use std::error::Error;
use std::fmt::Write as FmtWrite;
use std::path::PathBuf;
use std::sync::Arc;

use bytes::Bytes;
use futures::StreamExt;
use indicatif::{MultiProgress, ProgressBar, ProgressState, ProgressStyle};
use reqwest::Body;
use reqwest::multipart::Part;
use tokio::fs::File;
use tokio::io::AsyncRead;
use tokio_util::io::ReaderStream;

use crate::callback::CallbackFun;

/// A reader that tracks progress and updates a progress bar
/// as data is read from it using the `AsyncRead` trait.
///
/// # Fields
///
/// * `inner` - A boxed dynamic type that implements `AsyncRead`, `Unpin`, `Send`, and `Sync` traits.
/// * `pb` - An `Arc<ProgressBar>` instance used to display the progress of the read operation.
/// * `callback` - An optional `CallbackFun` instance for handling callbacks during the read process.
pub struct ProgressReader {
    inner: Box<dyn AsyncRead + Unpin + Send + Sync>,
    pb: Arc<ProgressBar>,
    callback: Option<CallbackFun>,
}

impl AsyncRead for ProgressReader {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        let start = buf.filled().len();
        let result = std::pin::Pin::new(&mut self.inner).poll_read(cx, buf);
        let bytes_read = buf.filled().len() - start;

        if bytes_read > 0 {
            self.pb.inc(bytes_read as u64);

            if let Some(callback) = &self.callback {
                callback.call(bytes_read as u64);
            }
        }

        result
    }
}

/// Creates a single-part body for a file upload.
///
/// This asynchronous function sets up a progress reader for the specified file,
/// creates a stream from the reader, and wraps the stream in a `reqwest::Body`
/// to be used in a file upload request.
///
/// # Arguments
///
/// * `file_path` - A reference to a `PathBuf` representing the path to the file to be uploaded.
/// * `multi_pb` - An `Arc<MultiProgress>` instance used to manage multiple progress bars.
/// * `callback` - An optional `CallbackFun` instance for handling callbacks during the upload process.
///
/// # Returns
///
/// A `Result` wrapping a `reqwest::Body` that contains the file stream, or a boxed `Error` if the setup fails.
pub async fn create_singlepart(
    file_path: &PathBuf,
    multi_pb: Arc<MultiProgress>,
    callback: Option<CallbackFun>,
) -> Result<Body, Box<dyn Error>> {
    let reader = setup_progress_reader(file_path, multi_pb, callback).await?;
    let stream = ReaderStream::new(reader).map(|result| result.map(Bytes::from));

    Ok(reqwest::Body::wrap_stream(stream))
}


/// Creates a multipart body for a file upload.
///
/// This asynchronous function sets up a progress reader for the specified file,
/// creates a stream from the reader, and wraps the stream in a `reqwest::Body`
/// to be used in a multipart file upload request.
///
/// # Arguments
///
/// * `file_path` - A reference to a `PathBuf` representing the path to the file to be uploaded.
/// * `multi_pb` - An `Arc<MultiProgress>` instance used to manage multiple progress bars.
/// * `callback` - An optional `CallbackFun` instance for handling callbacks during the upload process.
///
/// # Returns
///
/// A `Result` wrapping a `reqwest::multipart::Part` that contains the file stream, or a boxed `Error` if the setup fails.
pub async fn create_multipart(
    file_path: &PathBuf,
    multi_pb: Arc<MultiProgress>,
    callback: Option<CallbackFun>,
) -> Result<Part, Box<dyn std::error::Error>> {
    let reader = setup_progress_reader(file_path, multi_pb, callback).await?;
    let stream = ReaderStream::new(reader).map(|result| result.map(Bytes::from));

    // Create a multipart part
    let filename = file_path
        .to_str()
        .expect("The file path is invalid.")
        .rsplit('/')
        .next()
        .expect("The file path is invalid.");

    let part = Part::stream(reqwest::Body::wrap_stream(stream))
        .file_name(filename.to_string())
        .mime_str("application/octet-stream")?;

    Ok(part)
}

/// Sets up a progress reader for a file.
///
/// This asynchronous function opens the specified file, retrieves its length,
/// creates a progress bar, and wraps the file in a `ProgressReader` to track the progress
/// of the read operation.
///
/// # Arguments
///
/// * `file_path` - A reference to a `PathBuf` representing the path to the file to be read.
/// * `multi_pb` - An `Arc<MultiProgress>` instance used to manage multiple progress bars.
/// * `callback` - An optional `CallbackFun` instance for handling callbacks during the read process.
///
/// # Returns
///
/// A `Result` wrapping a `ProgressReader` that tracks the progress of the read operation,
/// or a boxed `Error` if the setup fails.
async fn setup_progress_reader(
    file_path: &PathBuf,
    multi_pb: Arc<MultiProgress>,
    callback: Option<CallbackFun>,
) -> Result<ProgressReader, Box<dyn Error>> {
    // Open the file and get its length
    let file = File::open(file_path).await?;
    let file_length = file.metadata().await?.len();

    // Create a progress bar and add it to the MultiProgress
    let pb: Arc<ProgressBar> = Arc::new(multi_pb.add(ProgressBar::new(file_length)));
    pb.set_style(
        ProgressStyle::with_template(
            "\n{spinner:.green} [{elapsed_precise}] {bar:.gray/black} {bytes}/{total_bytes} ({eta})\n",
        )
            .unwrap()
            .with_key("eta", |state: &ProgressState, w: &mut dyn FmtWrite| {
                write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
            })
            .progress_chars("■ "),
    );

    // Wrap the file in a ProgressReader to track progress
    let reader = ProgressReader {
        inner: Box::new(file),
        pb: Arc::clone(&pb),
        callback,
    };

    Ok(reader)
}