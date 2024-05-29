use colored::Colorize;
use lazy_static::lazy_static;
use rayon::prelude::*;

pub mod file;

lazy_static! {
    pub static ref COLOR: bool = atty::is(atty::Stream::Stdout);
}

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

    /// Return a pretty string representation of the bookmark
    pub fn pretty(&self) -> String {
        if *COLOR {
            format!(
                "{} - {} ({})",
                self.name.yellow(),
                self.url.blue(),
                self.tag_string(true)
            )
        } else {
            format!("{} - {} ({})", self.name, self.url, self.tag_string(false))
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