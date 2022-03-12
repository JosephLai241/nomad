//! Structs used during directory traversal.

/// Contains the path of the found item and its corresponding Git marker if applicable.
///
/// This struct is used to convert `DirEntry`s returned by the `Walk` object.
#[derive(Debug)]
pub struct FoundItem {
    /// The Git status marker indicating the change that was made to the file.
    pub marker: Option<String>,
    /// The start and end of the pattern match in the path.
    pub matched: Option<(usize, usize)>,
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
    /// The depth of the file relative to the root of the directory.
    pub depth: i32,
    /// Indicates whether this is a directory.
    pub is_dir: bool,
    /// Indicates whether this is a file.
    pub is_file: bool,
    /// The Git status marker indicating the change that was made to the file.
    pub marker: Option<String>,
    /// The start and end of the pattern match in the path.
    pub matched: Option<(usize, usize)>,
    /// The absolute filepath.
    pub path: String,
}

/// Contains metadata for `git branch` items.
///
/// This struct is used to convert Git branches into a struct containing metadata used for tree
/// building.
#[derive(Debug)]
pub struct FoundBranch {
    /// The full branch name.
    pub full_branch: String,
    /// Indicates whether this is the current branch.
    pub is_current_branch: bool,
    /// Indicates whether this branch points to `HEAD`.
    pub is_head: bool,
    /// The marker indicating whether this is the current branch.
    pub marker: Option<String>,
    /// The start and end of the pattern match in the branch name.
    pub matched: Option<(usize, usize)>,
    /// The upstream branch if it exists.
    pub upstream: Option<String>,
}

/// The `TransformFound` trait converts a `FoundBranch` into this struct for tree building.
///
/// These fields assume branch names are formatted like directory paths, ie.
/// `feature/something_new`.
#[derive(Debug)]
pub struct TransformedBranch {
    /// The branch name broken down into its individual components.
    pub components: Vec<String>,
    /// The depth of the branch relative to its components.
    pub depth: i32,
    /// The full branch name.
    pub full_branch: String,
    /// Indicates whether this is the current branch.
    pub is_current_branch: bool,
    /// Indicates whether this is the end of a branch name.
    pub is_end: bool,
    /// Indicates whether this branch points to `HEAD`.
    pub is_head: bool,
    /// Indicates whether the branch name has a parent name. For example, if the
    /// branch name is `feature/something_new`, the parent would be `feature`.
    pub is_parent: bool,
    /// The marker indicating whether this is the current branch.
    pub marker: Option<String>,
    /// The start and end of the pattern match in the branch name.
    pub matched: Option<(usize, usize)>,
    /// The upstream branch if it exists. This is also formatted if it points to
    /// `HEAD`.
    pub upstream: Option<String>,
}
