//! Normalizing errors for `nomad`.

use thiserror::Error;

use std::io;

/// Contains options for errors that may be raised throughout this program.
#[derive(Debug, Error)]
pub enum NomadError {
    /// Something went wrong when trying to `bat` a file.
    #[error("Bat error: {0}")]
    BatError(#[from] bat::error::Error),

    /// Something went wrong when trying to get the application-specific directories.
    #[error("Could not retrieve system application directories!")]
    ApplicationError,

    /// Something went wrong when opening a file with an editor.
    #[error("Unable to open the file with {editor}: {reason}")]
    EditorError {
        editor: String,
        #[source]
        reason: io::Error,
    },

    /// A generic formatted error.
    #[error(transparent)]
    Error(#[from] anyhow::Error),

    /// An invalid target was entered when attempting to run the Git blame subcommand.
    #[error("`git blame` can only be called on a single file")]
    GitBlameError,

    /// Something went wrong when running Git subcommands.
    #[error("{context:?}: {source:?}")]
    GitError {
        context: String,
        #[source]
        source: git2::Error,
    },

    /// Error for the `ignore` crate.
    #[error("Ignore error: {0}")]
    IgnoreError(#[from] ignore::Error),

    /// IO errors raised from the standard library (std::io).
    #[error("IO error: {0}")]
    IOError(#[from] io::Error),

    /// Something went wrong with the MPSC receiver.
    #[error("MPSC error: {0}")]
    MPSCError(#[from] std::sync::mpsc::RecvError),

    /// No items are available in Rootless mode.
    #[error("There are no items in this directory!")]
    NoItems,

    /// Nothing was found.
    #[error("No items were found!")]
    NothingFound,

    /// Nothing is currently selected in Rootless mode.
    #[error("Nothing is selected!")]
    NothingSelected,

    /// An invalid directory path is provided.
    #[error("{0} is not a directory!")]
    NotADirectory(String),

    /// Unable to edit the selected item because it is a directory.
    /// This is only used in Rootless mode.
    #[error("Unable to edit: The selected item is not a file!")]
    NotAFile,

    /// Raised when any path-related errors arise.
    #[error("Path error: {0}")]
    PathError(String),

    /// Plain Git error.
    #[error("{0}")]
    PlainGitError(#[from] git2::Error),

    /// Something went wrong while displaying a `ptree`.
    #[error("{context}: {source}")]
    PTreeError {
        context: String,
        #[source]
        source: io::Error,
    },

    /// Something went wrong when compiling a regex expression.
    #[error("{0}")]
    RegexError(#[from] regex::Error),

    /// Something went wrong when self-updating.
    #[error("Self-upgrade error: {0}")]
    SelfUpgradeError(#[from] self_update::errors::Error),

    /// Something went wrong when doing something with Serde JSON.
    #[error("Serde JSON error: {0}")]
    SerdeJSONError(#[from] serde_json::Error),

    /// Something went wrong when deserializing/serializing the TOML config file.
    #[error("TOML error: {0}")]
    TOMLError(#[from] toml::de::Error),

    /// Something went wrong when decoding to UTF-8.
    #[error("UTF-8 error: {0}")]
    UTF8Error(#[from] std::str::Utf8Error),
}
