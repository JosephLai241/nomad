//! Normalizing errors for `nomad`.

use thiserror::Error;

use std::io;

/// Contains options for errors that may be raised throughout this program.
#[derive(Debug, Error)]
pub enum NomadError {
    /// Something went wrong when trying to `bat` a file.
    #[error("Bat error: {0}")]
    BatError(#[from] bat::error::Error),

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

    /// An invalid directory path is provided.
    #[error("{0} is not a directory!")]
    NotADirectory(String),

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
    #[error("Regex error: {0}")]
    RegexError(#[from] regex::Error),

    /// Something went wrong when self-updating.
    #[error("Self-update error: {0}")]
    SelfUpdateError(#[from] self_update::errors::Error),

    /// Something went wrong when doing something with Serde JSON.
    #[error("Serde JSON error: {0}")]
    SerdeJSONError(#[from] serde_json::Error),
}
