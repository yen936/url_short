use std::fs;
use std::io;
use serde_json;
use crate::models::UrlMapping;

const DB_FILE: &str = "urls.json";

pub fn find_short_url_by_original(original_url: &str) -> io::Result<Option<String>> {
    let mappings = load_mappings()?;
    for mapping in mappings {
        if mapping.original_url == original_url {
            return Ok(Some(mapping.short_url));
        }
    }
    Ok(None)
}

pub fn load_mappings() -> io::Result<Vec<UrlMapping>> {
    let data = fs::read_to_string(DB_FILE).unwrap_or("[]".to_string());
    let mappings: Vec<UrlMapping> = serde_json::from_str(&data)?;
    Ok(mappings)
}

pub fn save_mapping(mapping: &UrlMapping) -> io::Result<()> {
    let mut mappings = load_mappings()?;
    mappings.push(mapping.clone());
    let json_data = serde_json::to_string(&mappings)?;
    fs::write(DB_FILE, json_data)?;
    Ok(())
}
