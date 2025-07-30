//! TCP socket utilities for handling streaming data connections.
//!
//! This module provides functionality for managing TCP socket connections
//! in the context of file uploads. It supports concurrent connections,
//! graceful shutdown, and data streaming through channels.

use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc,
};

use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
    sync::mpsc::Sender,
};
use tokio_stream::{wrappers::TcpListenerStream, StreamExt};

use crate::file::uploadfile::BUFFER_SIZE;

/// Spawns a background task to handle Unix socket connections.
///
/// This method creates a background task that listens for both regular
/// connections and finish signals, managing the lifecycle of the Unix
/// socket stream appropriately.
///
/// The function uses `tokio::select!` to concurrently handle:
/// - Regular data connections on the main listener
/// - Finish signals on the finish listener for graceful shutdown
///
/// # Arguments
/// * `listener` - The main Unix listener for data connections.
/// * `finish_listener` - The finish listener for shutdown signals.
/// * `tx` - The sender channel to forward data chunks.
/// * `should_cancel` - Atomic flag for cancellation signaling.
/// * `active_connections` - Counter for tracking active connections.
pub(crate) fn spawn_connection_handler(
    listener: TcpListener,
    finish_listener: TcpListener,
    tx: Sender<Vec<u8>>,
    should_cancel: Arc<AtomicBool>,
    active_connections: Arc<AtomicUsize>,
) {
    let should_cancel_clone = Arc::clone(&should_cancel);
    let active_connections_clone = Arc::clone(&active_connections);

    tokio::spawn(async move {
        let mut incoming = TcpListenerStream::new(listener);
        let mut finish_incoming = TcpListenerStream::new(finish_listener);

        loop {
            tokio::select! {
                // Handle regular connections
                conn_result = incoming.next() => {
                    if let Some(Ok(stream)) = conn_result {
                        if !should_cancel_clone.load(Ordering::Relaxed) {
                            handle_new_connection(stream, tx.clone(), Arc::clone(&active_connections_clone));
                        }
                    } else if conn_result.is_none() {
                        break; // No more incoming connections
                    }
                }

                // Handle finish listener connections
                finish_result = finish_incoming.next() => {
                    if finish_result.is_some() {
                        handle_finish_signal(should_cancel_clone, active_connections_clone, tx).await;
                        break;
                    }
                }
            }
        }
    });
}

/// Handles a new Unix socket connection.
///
/// Spawns a separate task for each connection to handle data streaming
/// concurrently while tracking the number of active connections.
///
/// This function:
/// 1. Increments the active connection counter
/// 2. Spawns an async task to handle the connection's data stream
/// 3. Decrements the counter when the connection ends
///
/// # Arguments
/// * `stream` - The Unix stream for the new connection.
/// * `tx` - The sender channel to forward data chunks.
/// * `active_connections` - Counter for tracking active connections.
///
/// # Note
/// Each connection is handled in its own async task to allow for
/// concurrent data streaming from multiple sources.
fn handle_new_connection(
    stream: TcpStream,
    tx: Sender<Vec<u8>>,
    active_connections: Arc<AtomicUsize>,
) {
    // Increment active connection count
    active_connections.fetch_add(1, Ordering::Relaxed);

    // Spawn a separate task for each connection
    tokio::spawn(async move {
        connection_streamer(stream, tx).await;
        // Decrement when connection ends
        active_connections.fetch_sub(1, Ordering::Relaxed);
    });
}

/// Handles the finish signal for graceful shutdown.
///
/// Sets the cancellation flag, waits for all active connections to finish,
/// and then drops the sender channel to signal the end of the stream.
///
/// This function implements a graceful shutdown sequence:
/// 1. Sets the cancellation flag to prevent new connections
/// 2. Waits for all active connections to complete their data transfer
/// 3. Drops the sender channel to signal stream completion
///
/// # Arguments
/// * `should_cancel` - Atomic flag for cancellation signaling.
/// * `active_connections` - Counter for tracking active connections.
/// * `tx` - The sender channel to be dropped after shutdown.
///
/// # Note
/// The function polls the active connections counter every 10ms
/// to ensure all connections have finished before completing shutdown.
async fn handle_finish_signal(
    should_cancel: Arc<AtomicBool>,
    active_connections: Arc<AtomicUsize>,
    tx: Sender<Vec<u8>>,
) {
    // Set cancellation flag
    should_cancel.store(true, Ordering::Relaxed);

    // Wait for all active connections to finish
    while active_connections.load(Ordering::Relaxed) > 0 {
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    }

    // Drop the tx to signal end of stream
    drop(tx);
}

/// Streams data from a Unix socket connection to a channel.
///
/// This function continuously reads data from the Unix socket stream
/// and forwards it through the provided sender channel. It handles
/// the low-level details of reading from the socket and managing
/// the buffer.
///
/// The function will:
/// - Read data in chunks using a buffer of size `BUFFER_SIZE`
/// - Send each chunk through the provided channel
/// - Handle EOF and connection errors gracefully
/// - Exit when the connection is closed or an error occurs
///
/// # Arguments
/// * `stream` - The Unix socket stream to read from.
/// * `tx` - The sender channel to forward data chunks to.
///
/// # Behavior
/// - Returns when EOF is reached (0 bytes read)
/// - Returns when a read error occurs
/// - Returns when the receiver channel is dropped
///
/// # Note
/// This function is designed to be called from within an async task
/// for each individual connection to enable concurrent streaming.
async fn connection_streamer(stream: TcpStream, tx: Sender<Vec<u8>>) {
    let mut stream = stream;
    let mut buffer = vec![0; *BUFFER_SIZE];

    loop {
        match stream.read(&mut buffer).await {
            Ok(0) => break, // EOF - connection closed by client
            Ok(bytes_read) => {
                // Create a chunk from the read data
                let chunk = buffer[..bytes_read].to_vec();
                if tx.send(chunk).await.is_err() {
                    return; // Receiver dropped - stop streaming
                }
            }
            Err(_) => break, // Connection error - stop streaming
        }
    }
}
