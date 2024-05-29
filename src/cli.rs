use clap::{Parser, Subcommand};
use quickmark::{
    file::{load_bulk, save_bulk},
    Bookmark,
};
use rayon::prelude::*;

#[derive(Parser)]
#[command(about = "A simple bookmark manager")]
struct App {
    /// File to store bookmarks
    #[arg(short, long, default_value = "bookmarks.qm")]
    filename: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List bookmarks
    List {
        /// Number of bookmarks to list (default: all)
        limit: Option<usize>,
    },
    /// Add a new bookmark
    Add {
        /// Name of the bookmark
        name: String,
        /// URL of the bookmark
        url: String,
        /// Tags for the bookmark
        tags: Vec<String>,
    },
    /// Remove a bookmark by index
    Remove {
        /// Index of the bookmark to remove
        index: usize,
    },
    /// Search for bookmarks by tag (default) or fuzzy-find
    Search {
        /// Query to match against
        query: String,

        /// Fuzzy search the whole bookmark. Compares the query against each bookmark's serialized form
        #[clap(short, long)]
        fuzzy: bool,
    },
    /// Open a bookmark in the default browser
    Open {
        /// Index of the bookmark to open
        index: usize,
    },
}

fn cmd_list(filename: &str, limit: Option<usize>) {
    let bookmarks = load_bulk(filename).unwrap_or_default();
    bookmarks.iter().take(limit.unwrap_or(bookmarks.len())).for_each(|b| {
        println!("{}", b.pretty());
    });
}

fn cmd_add(filename: &str, name: &str, url: &str, tags: Vec<String>) {
    let mut bookmarks = load_bulk(filename).unwrap_or_default();
    bookmarks.push(Bookmark {
        name: name.to_owned(),
        url: url.to_owned(),
        tags,
    });
    save_bulk(filename, bookmarks).unwrap();
}

fn cmd_remove(filename: &str, index: usize) {
    let mut bookmarks = load_bulk(filename).unwrap_or_default();
    if index < bookmarks.len() {
        bookmarks.remove(index);
        save_bulk(filename, bookmarks).unwrap();
    } else {
        eprintln!("Index out of bounds");
    }
}

fn cmd_search(filename: &str, query: &str, fuzzy: bool) {
    let bookmarks = load_bulk(filename).unwrap_or_default();
    let query = query.to_lowercase();
    bookmarks
        .par_iter()
        .filter(|b| {
            if fuzzy {
                b.serialize().to_lowercase().contains(&query)
            } else {
                b.tags.iter().any(|tag| tag.contains(&query))
            }
        })
        .for_each(|b| {
            println!("{}", b.pretty());
        });
}

fn cmd_open(filename: &str, index: usize) {
    let bookmarks = load_bulk(filename).unwrap_or_default();
    if index < bookmarks.len() {
        let _ = webbrowser::open(&bookmarks[index].url);
    } else {
        eprintln!("Index out of bounds");
    }
}

pub fn run() {
    let args = App::parse();
    if let Some(command) = args.command {
        match command {
            Commands::Open { index } => cmd_open(&args.filename, index),
            Commands::List { limit } => cmd_list(&args.filename, limit),
            Commands::Add { name, url, tags } => cmd_add(&args.filename, &name, &url, tags),
            Commands::Remove { index } => cmd_remove(&args.filename, index),
            Commands::Search { query, fuzzy } => cmd_search(&args.filename, &query, fuzzy),
        }
    } else {
        eprintln!("No command provided. Try `qm help`");
    }
}
