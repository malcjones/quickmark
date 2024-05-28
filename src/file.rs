use crate::{deserialize_multi, serialize_vec, Bookmark};
use std::io::Write;

/// Load a list of bookmarks from a file
pub fn load_multi(path: &str) -> Result<Vec<Bookmark>, std::io::Error> {
    let data = std::fs::read_to_string(path)?;
    Ok(deserialize_multi(data))
}

/// Save a vector of bookmarks to a file
pub fn save_multi(path: &str, bookmarks: Vec<Bookmark>) -> Result<(), std::io::Error> {
    let mut file = std::fs::File::create(path)?;
    file.write_all(serialize_vec(bookmarks).as_bytes())
}
