//! Structs used when serializing/deserializing data from JSON.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Store directory items with an associated index number.
#[derive(Debug, Deserialize, Serialize)]
pub struct Contents {
    pub items: HashMap<String, String>,
}
