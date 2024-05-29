use crate::{Bookmark};
use std::io::Write;
use rayon::prelude::*;

/// Load a list of bookmarks from a file
pub fn load_bulk<'a>(path: &'a str) -> Result<Vec<Bookmark>, std::io::Error> {
    let data = std::fs::read_to_string(path)?;
    Ok(data.par_lines().filter_map(Bookmark::deserialize).collect())
}

/// Save a vector of bookmarks to a file
pub fn save_bulk(path: &str, bookmarks: Vec<Bookmark>) -> Result<(), std::io::Error> {
    let mut file = std::fs::File::create(path)?;
    let data = bookmarks
        .par_iter()
        .map(Bookmark::serialize)
        .collect::<Vec<String>>()
        .join("\n")
        .as_bytes()
        .to_owned();
    file.write_all(&data)?;
    Ok(())
}
