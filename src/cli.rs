use clap::{Parser, Subcommand};
use quickmark::{
    file::{load_multi, save_multi},
    Bookmark,
};
use rayon::prelude::*;

#[derive(Parser)]
#[command(about = "A simple bookmark manager")]
struct Cli {
    /// File to store bookmarks
    #[arg(short, long, default_value = "bookmarks.qm")]
    filename: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List the all, or first N, bookmarks
    List {
        /// Number of bookmarks to list
        n: Option<usize>,
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
}

fn cmd_list(filename: &str, n: Option<usize>) {
    let bookmarks = load_multi(filename).unwrap_or_default();
    bookmarks[..n.unwrap_or(bookmarks.len())]
        .iter()
        .enumerate()
        .for_each(|(i, b)| {
            println!("{i}. {}", b.pretty());
        });
}

fn cmd_add(filename: &str, name: &str, url: &str, tags: Vec<String>) {
    let mut bookmarks = load_multi(filename).unwrap_or_default();
    bookmarks.push(Bookmark::new(name, url, tags));
    save_multi(filename, bookmarks).unwrap();
}

fn cmd_remove(filename: &str, index: usize) {
    let mut bookmarks = load_multi(filename).unwrap_or_default();
    if index < bookmarks.len() {
        bookmarks.remove(index);
        save_multi(filename, bookmarks).unwrap();
    } else {
        eprintln!("Index out of bounds");
    }
}

fn cmd_search(filename: &str, query: &str, fuzzy: bool) {
    let bookmarks = load_multi(filename).unwrap_or_default();
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
pub fn run_cli() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::List { n }) => cmd_list(&args.filename, n),
        Some(Commands::Add { name, url, tags }) => cmd_add(&args.filename, &name, &url, tags),
        Some(Commands::Remove { index }) => cmd_remove(&args.filename, index),
        Some(Commands::Search { query, fuzzy }) => cmd_search(&args.filename, &query, fuzzy),
        None => eprintln!("No command provided"),
    }
}
