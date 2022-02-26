//! Structs used during directory traversal.

/// Contains the path of the found item and its corresponding Git marker if applicable.
///
/// This struct is used to convert `DirEntry`s returned by the `Walk` object.
#[derive(Debug)]
pub struct FoundItem {
    /// The Git status marker indicating the change that was made to the file.
    pub marker: Option<String>,
    /// The filepath.
    pub path: String,
}

/// Contains metadata for each path.
///
/// The `TransformFound` trait converts a `FoundItem` into this struct for tree building.
#[derive(Debug)]
pub struct TransformedItem {
    /// The filepath broken down into its individual components.
    pub components: Vec<String>,
    /// The depth of the file relative to the root of the Git repository.
    pub depth: i32,
    /// Indicates whether this is a directory.
    pub is_dir: bool,
    /// Indicates whether this is a file.
    pub is_file: bool,
    /// The Git status marker indicating the change that was made to the file.
    pub marker: Option<String>,
    /// The absolute filepath.
    pub path: String,
}
