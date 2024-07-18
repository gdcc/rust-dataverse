use std::fmt::Write as FmtWrite;
use std::sync::Arc;

use bytes::Bytes;
use futures::StreamExt;
use indicatif::{MultiProgress, ProgressBar, ProgressState, ProgressStyle};
use reqwest::multipart::Part;
use tokio::fs::File;
use tokio::io::AsyncRead;
use tokio_util::io::ReaderStream;

use crate::callback::CallbackFun;

pub async fn create_multipart(
    file_path: &str,
    multi_pb: Arc<MultiProgress>,
    callback: Option<CallbackFun>,
) -> Result<Part, Box<dyn std::error::Error>> {
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

    // Create a stream from the ProgressReader
    let stream = ReaderStream::new(reader).map(|result| result.map(Bytes::from));

    // Create a multipart part
    let filename = file_path
        .rsplit('/')
        .next()
        .expect("The file path is invalid.");
    let part = Part::stream(reqwest::Body::wrap_stream(stream))
        .file_name(filename.to_string())
        .mime_str("application/octet-stream")?;

    Ok(part)
}

// A reader that tracks progress and updates a progress bar
// as data is read from it using the AsyncRead trait.
struct ProgressReader {
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