use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct UrlMapping {
    pub short_url: String,
    pub original_url: String,
}
