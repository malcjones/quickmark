# quickmark

tiny program for manipulating a list of urls with names & tags

# usage

```
cargo build --release
./target/release/qm help
```

Keep in mind that any commands that mutate the bookmarks file truncate it.
Your bookmarks will be preserved, but any whitespace or comments will be lost.
This is a feature. Modification of the bookmarks file is only provided as a convenience
(for large files, or if managing a link collection with a script). Managing the bookmarks
file with a text editor is the intended use case.

# format

```
# <- comments start with a hash
bookmark_name|url|tag1,tag2, tag3
# whitespace is kept in the name & url, but tags are trimmed
```