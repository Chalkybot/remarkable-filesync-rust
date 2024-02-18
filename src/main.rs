use std::path::{Path, PathBuf};
use sha2::Sha256;
use chrono::{Utc, DateTime};

struct RmObject<'a> {
    object_type: u8, // I'm using a u8 as it's a very small datatype. I might switch this out in the future.
    created: DateTime<Utc>,
    modified: DateTime<Utc>,
    name: &'a str,
    hash: Sha256,
    uuid: &'a str,
    content_path: PathBuf,
    local_path: PathBuf,
    metadata_path: PathBuf,
    pagedata_path: PathBuf,
}

impl RmObject<'_> {
    fn new(metadata_path: PathBuf, possible_data_path: Option<PathBuf>) -> Self {
        // todo: Proper error handling.
        let metadata_contents = match std::fs::read_to_string(metadata_path) {
            Ok(contents) => contents,
            Err(e) => panic!("{:?}", e),
        };

        // If the content path is passed, we can expect it to be .epub or .pdf. If it is not, we default to .content.
        let data_path: PathBuf = match possible_data_path {
            None => {
                let mut temp_path = metadata_path.clone();
                temp_path.set_extension("content");
                temp_path
            },
            Some(path) => path, 
        };

        

    }

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
