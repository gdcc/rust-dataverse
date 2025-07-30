use std::path::Path;

use colored::Colorize;
use structopt::StructOpt;
use tokio::{
    net::{TcpListener, UnixListener},
    runtime::Runtime,
};

use crate::{
    cli::base::{evaluate_and_print_response, Matcher},
    client::BaseClient,
    file::{uploadfile::tcp_listener_to_upload_file, UploadFile},
    prelude::{
        dataset::{upload::UploadBody, upload_file_to_dataset},
        Identifier,
    },
};

/// Subcommands for managing pipes to upload files to a dataset
#[derive(StructOpt, Debug)]
#[structopt(about = "Open a pipe to upload a file to a dataset from anywhere")]
pub enum PipeSubCommand {
    /// Create a new Unix pipe
    #[structopt(about = "Create a new Unix pipe to upload a file to a dataset")]
    Unix {
        #[structopt(short, long, help = "Name of the socket to open")]
        socket: String,

        #[structopt(short, long, help = "Identifier of the dataset to upload")]
        id: Identifier,

        #[structopt(short, long, help = "Dataverse path to the file to upload")]
        path: String,
    },

    /// Create a new TCP pipe to upload a file to a dataset
    #[structopt(about = "Create a new TCP pipe to upload a file to a dataset")]
    TCP {
        #[structopt(
            short = "P",
            long,
            help = "Port to open for data",
            default_value = "12788"
        )]
        data_port: u16,

        #[structopt(
            short = "F",
            long,
            help = "Port to open for finish",
            default_value = "12789"
        )]
        finish_port: u16,

        #[structopt(short, long, help = "Identifier of the dataset to upload")]
        id: Identifier,

        #[structopt(short, long, help = "Dataverse path to the file to upload")]
        path: String,
    },
}

impl Matcher for PipeSubCommand {
    /// Process the pipe subcommand, creating the appropriate listener and uploading the file
    fn process(self, client: &BaseClient) {
        let runtime = Runtime::new().unwrap();
        match self {
            PipeSubCommand::Unix { socket, id, path } => {
                runtime.block_on(async {
                    let listener = create_socket(&socket).expect("Failed to create socket");
                    let mut file: UploadFile = listener.into();

                    let body = add_file_info(&mut file, &path);
                    let response =
                        upload_file_to_dataset(&client, id, file, Some(body), None).await;
                    evaluate_and_print_response(response);
                });
            }
            PipeSubCommand::TCP {
                data_port,
                finish_port,
                id,
                path,
            } => {
                runtime.block_on(async {
                    let (listener, finish_listener) = create_tcp_listener(data_port, finish_port)
                        .await
                        .expect("Failed to create TCP listener");
                    let mut file: UploadFile =
                        tcp_listener_to_upload_file(listener, finish_listener);

                    let body = add_file_info(&mut file, &path);
                    let response =
                        upload_file_to_dataset(&client, id, file, Some(body), None).await;
                    evaluate_and_print_response(response);
                });
            }
        }
    }
}

/// Creates a Unix socket listener for file uploads
/// 
/// This function creates a Unix domain socket at `/tmp/{socket_name}.sock` and removes
/// any existing socket file at that path. It prints information about where to send
/// data and how to finish the upload.
/// 
/// # Arguments
/// 
/// * `socket_name` - The name of the socket to create
/// 
/// # Returns
/// 
/// Returns a `Result` containing the `UnixListener` on success, or an `std::io::Error` on failure.
pub fn create_socket(socket_name: &str) -> Result<UnixListener, std::io::Error> {
    let socket_path = format!("/tmp/{}.sock", socket_name);

    if Path::new(&socket_path).exists() {
        if let Err(e) = std::fs::remove_file(&socket_path) {
            eprintln!("Warning: Failed to remove existing socket file: {}", e);
        }
    }

    let finish_address = format!("{}.finish", socket_path);

    println!("Starting UNIX upload on socket\n",);
    println!("  » Push data to {}", socket_path.cyan().bold());
    println!("  » Finish upload at {}\n", finish_address.cyan().bold());

    UnixListener::bind(socket_path)
}

/// Creates TCP listeners for file uploads
/// 
/// This function creates two TCP listeners: one for receiving data and another for
/// signaling upload completion. Both listeners bind to localhost (127.0.0.1) on
/// the specified ports.
/// 
/// # Arguments
/// 
/// * `data_port` - The port number for the data listener
/// * `finish_port` - The port number for the finish signal listener
/// 
/// # Returns
/// 
/// Returns a `Result` containing a tuple of `(TcpListener, TcpListener)` on success,
/// or an `std::io::Error` on failure.
pub async fn create_tcp_listener(
    data_port: u16,
    finish_port: u16,
) -> Result<(TcpListener, TcpListener), std::io::Error> {
    let data_address = format!("127.0.0.1:{}", data_port);
    let finish_address = format!("127.0.0.1:{}", finish_port);

    println!("Starting local TCP upload\n");
    println!("  » Push data to {}", data_address.cyan().bold());
    println!("  » Finish upload at {}\n", finish_address.cyan().bold());

    let listener = TcpListener::bind(data_address).await?;
    let finish_listener = TcpListener::bind(finish_address).await?;

    Ok((listener, finish_listener))
}

/// Adds file information to an UploadFile and creates an UploadBody
/// 
/// This function extracts the filename and directory path from the provided path,
/// updates the UploadFile with this information, and creates an UploadBody with
/// the directory label.
/// 
/// # Arguments
/// 
/// * `file` - A mutable reference to the UploadFile to update
/// * `path` - The file path string to extract information from
/// 
/// # Returns
/// 
/// Returns an `UploadBody` with the directory label set.
fn add_file_info(file: &mut UploadFile, path: &str) -> UploadBody {
    let path = Path::new(&path);
    let name = path.file_name().unwrap().to_str().unwrap();
    let dir = path.parent().map(|p| p.to_path_buf()).unwrap();
    file.name = name.to_string();
    file.dir = Some(dir.clone());

    UploadBody {
        directory_label: Some(dir.to_str().unwrap().to_string()),
        ..Default::default()
    }
}
