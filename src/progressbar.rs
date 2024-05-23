use indicatif::{MultiProgress, ProgressBar, ProgressState, ProgressStyle};
use reqwest::blocking::multipart::Part;
use std::fmt::Write;
use std::fs::File;
use std::io::{self, Read};
use std::sync::Arc;

// This function is used in the RequestType::Multipart::build_form_request method
// to wrap a file in a progress bar and return a multipart part.
pub fn wrap_progressbar(
    file_path: &str,
    multi_pb: &MultiProgress,
) -> Result<Part, Box<dyn std::error::Error>> {
    // Open the file and get its length
    let file = File::open(file_path)?;
    let file_length = file.metadata()?.len();

    // Create a progress bar and add it to the MultiProgress
    let pb: Arc<ProgressBar> = Arc::new(multi_pb.add(ProgressBar::new(file_length)));
    pb.set_style(
        ProgressStyle::with_template(
            "\n{spinner:.green} [{elapsed_precise}] {bar:.gray/black} {bytes}/{total_bytes} ({eta})\n",
        )
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
        })
        .progress_chars("■ "),
    );

    // Wrap the file in a ProgressReader to track progress
    let reader = ProgressReader {
        inner: Box::new(file),
        pb: Arc::clone(&pb),
    };

    // Create a multipart part
    let filename = file_path
        .rsplit('/')
        .next()
        .expect("The file path is invalid.");
    let part = Part::reader(reader)
        .file_name(filename.to_string())
        .mime_str("application/octet-stream")?;

    Ok(part)
}

// A reader that tracks progress and updates a progress bar
// as data is read from it using the Read trait.
struct ProgressReader {
    inner: Box<dyn Read + Send>,
    pb: Arc<ProgressBar>,
}

impl Read for ProgressReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let bytes_read = self.inner.read(buf)?;
        self.pb.inc(bytes_read as u64);
        Ok(bytes_read)
    }
}
