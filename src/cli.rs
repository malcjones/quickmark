use clap::{Parser, Subcommand};
use quickmark::{file::{load_multi, save_multi}, Bookmark};

#[derive(Parser)]
#[command(about = "A simple bookmark manager")]
struct Cli {
    /// File to store bookmarks
    #[arg(short, long)]
    filename: Option<String>,

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
    /// Search for bookmarks by tag
    Search {
        /// Tag to search for
        tag: String,
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

fn cmd_search(filename: &str, tag: &str) {
    load_multi(filename).unwrap_or_default()
        .iter()
        .enumerate()
        .filter(|(_, b)| b.tags.contains(&tag.to_owned()))
        .for_each(|(i, b)| {
            println!("{i}. {}", b.pretty());
        });
}

pub fn run_cli() {
    let args = Cli::parse();
    let filename = args.filename.unwrap_or("bookmarks.qm".to_owned());
    match args.command {
        Some(Commands::List { n }) => cmd_list(&filename, n),
        Some(Commands::Add { name, url, tags }) => cmd_add(&filename, &name, &url, tags),
        Some(Commands::Remove { index }) => cmd_remove(&filename, index),
        Some(Commands::Search { tag }) => cmd_search(&filename, &tag),
        None => eprintln!("No command provided"),
    }
}