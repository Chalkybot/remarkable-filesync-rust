use std::path::PathBuf;
use sha2::Sha256;
use chrono::{Utc, DateTime};

struct RmObject<'a> {
    object_type: u8, // I'm using a u8 as it's a very small datatype. I might switch this out in the future.
    created: DateTime<Utc>,
    modified: DateTime<Utc>,
    parent: &'a str,
    name: &'a str,
    hash: Sha256,
    uuid: &'a str,
    content_path: PathBuf,
    local_path: PathBuf,
    metadata_path: PathBuf,
    pagedata_path: PathBuf,
}

struct RmLibrary<'a> {
    total_amount: u32,
    book_amount: u32,
    note_amount: u32,
    books: Vec<RmObject<'a>>,
    notes: Vec<RmObject<'a>>,
}

fn fetch_library<'a>() -> Result<RmLibrary<'a>, Box<dyn std::error::Error>> {
    Err("placeholder".into())
}


fn main() {
}
