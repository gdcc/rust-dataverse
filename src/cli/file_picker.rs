use std::path::PathBuf;

use crate::file::UploadFile;

/// Configuration options for file selection.
///
/// This struct controls how files are selected, including whether multiple files can be picked
/// and whether to use a GUI dialog or command-line prompt.
#[derive(Debug, Clone)]
pub struct FilePickerOptions {
    /// If `true`, allow selecting multiple files.
    pub multi: bool,
    /// If `true`, force TTY prompt instead of GUI dialog.
    pub no_gui: bool,
}

impl Default for FilePickerOptions {
    /// Returns the default configuration for file picking (single file, GUI allowed).
    fn default() -> Self {
        Self {
            multi: false,
            no_gui: false,
        }
    }
}

impl FilePickerOptions {
    /// Creates a new `FilePickerOptions` with default settings (single file, GUI allowed).
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates options for multiple file selection.
    pub fn as_multi() -> Self {
        Self {
            multi: true,
            no_gui: false,
        }
    }

    /// Creates options for single file selection.
    pub fn as_single() -> Self {
        Self::default()
    }

    /// Sets whether to allow multiple file selection.
    pub fn multi(mut self, multi: bool) -> Self {
        self.multi = multi;
        self
    }

    /// Sets whether to disable the GUI file picker.
    pub fn no_gui(mut self, no_gui: bool) -> Self {
        self.no_gui = no_gui;
        self
    }
}

/// The result of a file picking operation.
///
/// This enum represents either a single file or multiple files selected by the user.
pub enum FilePickerResult {
    /// A single file was selected.
    Single(PathBuf),
    /// Multiple files were selected.
    Multiple(Vec<PathBuf>),
}

impl From<Vec<PathBuf>> for FilePickerResult {
    /// Converts a vector of `PathBuf` into a `FilePickerResult::Multiple`.
    fn from(files: Vec<PathBuf>) -> Self {
        FilePickerResult::Multiple(files)
    }
}

impl From<PathBuf> for FilePickerResult {
    /// Converts a single `PathBuf` into a `FilePickerResult::Single`.
    fn from(file: PathBuf) -> Self {
        FilePickerResult::Single(file)
    }
}

impl Into<Vec<UploadFile>> for FilePickerResult {
    /// Converts a `FilePickerResult` into a vector of `UploadFile`.
    fn into(self) -> Vec<UploadFile> {
        match self {
            FilePickerResult::Single(path) => vec![UploadFile::from(path)],
            FilePickerResult::Multiple(paths) => paths.into_iter().map(UploadFile::from).collect(),
        }
    }
}

impl Into<UploadFile> for FilePickerResult {
    /// Converts a `FilePickerResult::Single` into an `UploadFile`.
    ///
    /// # Panics
    ///
    /// Panics if called on `FilePickerResult::Multiple`.
    fn into(self) -> UploadFile {
        match self {
            FilePickerResult::Single(path) => UploadFile::from(path),
            FilePickerResult::Multiple(_) => {
                panic!("Multiple files are not supported for this operation.");
            }
        }
    }
}

/// Picks a file for upload, converting the result to the specified type.
///
/// This is a convenience wrapper around `pick_files` that handles conversion to the desired type.
///
/// # Arguments
///
/// * `options` - The file picker configuration options.
/// * `files` - A vector of pre-selected file paths.
///
/// # Returns
///
/// An `Option<T>` containing the converted result, or `None` if no files were selected.
pub fn pick_upload_file<T>(options: &FilePickerOptions, files: Vec<PathBuf>) -> Option<T>
where
    FilePickerResult: Into<T>,
{
    pick_files(options, files).map(|result| result.into())
}

/// Picks files based on the provided options and optional pre-selected files.
///
/// The selection priority is as follows:
/// 1. If `files` contains paths, use them directly.
/// 2. If GUI is allowed and available, show GUI file picker dialog.
/// 3. Otherwise, use TTY prompt.
///
/// # Arguments
///
/// * `options` - The file picker configuration options.
/// * `files` - A vector of pre-selected file paths.
///
/// # Returns
///
/// An `Option<FilePickerResult>` containing the selected files, or `None` if no files were selected.
pub fn pick_files(options: &FilePickerOptions, files: Vec<PathBuf>) -> Option<FilePickerResult> {
    if !files.is_empty() {
        return Some(files.into());
    }

    #[cfg(any(target_os = "macos", target_os = "windows"))]
    {
        if !options.no_gui && gui_probably_available() {
            if let Some(result) = pick_files_gui(options.multi) {
                return Some(result);
            }
            eprintln!("No file selected via GUI; falling back to TTY prompt…");
        }
    }

    Some(pick_file_tty(options.multi).into())
}

/// Checks if a GUI is probably available on the current platform.
///
/// On macOS and Windows, this always returns `true`.
/// On Linux and other platforms, this returns `false` (forcing TTY prompts).
fn gui_probably_available() -> bool {
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    {
        true
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        false
    }
}

/// Shows a GUI file picker dialog to the user.
///
/// If `multi` is `true`, allows selecting multiple files. Otherwise, only a single file can be selected.
///
/// This function is only available on macOS and Windows.
///
/// # Arguments
///
/// * `multi` - Whether to allow multiple file selection.
///
/// # Returns
///
/// An `Option<FilePickerResult>` containing the selected files, or `None` if no files were selected.
#[cfg(any(target_os = "macos", target_os = "windows"))]
fn pick_files_gui(multi: bool) -> Option<FilePickerResult> {
    let dlg = rfd::FileDialog::new();

    if multi {
        dlg.pick_files().map(|files| files.into())
    } else {
        dlg.pick_file().map(|file| file.into())
    }
}

/// Prompts the user for file path(s) via TTY (command-line).
///
/// If `multi` is `true`, prompts for comma-separated file paths. Otherwise, prompts for a single file path
/// and validates that the path exists.
///
/// # Arguments
///
/// * `multi` - Whether to allow multiple file selection.
///
/// # Returns
///
/// A vector of `PathBuf` containing the selected file paths.
fn pick_file_tty(multi: bool) -> Vec<PathBuf> {
    use inquire::{validator::Validation, Text};

    if multi {
        let input = Text::new("Enter file paths (comma-separated):")
            .prompt()
            .unwrap_or_default();
        input
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(PathBuf::from)
            .collect()
    } else {
        let path_str = Text::new("Enter a file path:")
            .with_validator(|input: &str| {
                if PathBuf::from(input).exists() {
                    Ok(Validation::Valid)
                } else {
                    Ok(Validation::Invalid("Path does not exist".into()))
                }
            })
            .prompt()
            .unwrap_or_default();

        if path_str.is_empty() {
            vec![]
        } else {
            vec![PathBuf::from(path_str)]
        }
    }
}
