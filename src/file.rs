use crate::Bookmark;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};

/// Load a list of bookmarks from a file
pub fn load_bulk(path: &str) -> io::Result<Vec<Bookmark>> {
    let file = std::fs::File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader
        .lines()
        .map(|line| line.unwrap())
        .filter_map(|line| Bookmark::deserialize(&line))
        .collect())
}

pub fn save_bulk(path: &str, bookmarks: Vec<Bookmark>) -> io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    let data = bookmarks
        .iter()
        .map(|bookmark| bookmark.serialize() + "\n")
        .collect::<String>();
    writer.write_all(data.as_bytes())
}
