use rayon::prelude::*;
use colored::Colorize;

pub mod file;

pub struct Bookmark {
    pub name: String,
    pub url: String,
    pub tags: Vec<String>,
}

impl Bookmark {
    /// Deserialize a `&str` to a Bookmark
    /// Returns `None` if the line is empty, begins with `#`, or is missing a field.
    ///
    /// Format:
    /// `"name|url|tag1, tag2" -> { name,url, vec!["tag1", "tag2"}`
    pub fn deserialize(line: &str) -> Option<Self> {
        if line.is_empty() || line.starts_with('#') {
            return None;
        }
        let mut parts = line.split('|').map(|field| field.to_owned());
        Some(Bookmark {
            name: parts.next()?,
            url: parts.next()?,
            tags: parts
                .next()?
                .split(',')
                .map(|tag| tag.trim().to_owned())
                .collect(),
        })
    }

    pub fn serialize(&self) -> String {
        format!("{}|{}|{}", &self.name, &self.url, self.tag_string(false))
    }

    pub fn pretty(&self) -> String {
        format!("{} [{}] -> {}", self.name, self.tag_string(true), self.url.blue())
    }


    pub fn new<'a>(name: &str, url: &str, tags: Vec<String>) -> Self {
        Self {
            name: name.to_owned(),
            url: url.to_owned(),
            tags,
        }
    }

    pub fn tag_string(&self, pretty: bool) -> String {
        if pretty {
            self.tags
                .iter()
                .map(|tag| tag.green().to_string())
                .collect::<Vec<String>>()
                .join(", ")
        } else {
            self.tags.join(",")
        }
    }
}


/// Serialize a vector of bookmarks into a single string, one per line
pub fn serialize_vec(bookmarks: Vec<Bookmark>) -> String {
    bookmarks
        .par_iter()
        .map(|b| b.serialize())
        .collect::<Vec<String>>()
        .join("\n")
}

/// Deserialize bookmarks encoded as a string, one per line
pub fn deserialize_multi(data: String) -> Vec<Bookmark> {
    data.par_lines()
        .filter_map(Bookmark::deserialize)
        .collect()
}