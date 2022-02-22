//! Structs used when serializing/deserializing data from JSON.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Store all directory items.
#[derive(Debug, Deserialize, Serialize)]
pub struct Contents {
    /// Contains labeled directory paths.
    pub labeled: HashMap<String, String>,
    /// Contains numbered directory items.
    pub numbered: HashMap<String, String>,
}
