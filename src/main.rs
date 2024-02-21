use std::path::PathBuf;
use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};
use itertools::Itertools;
use clap::Parser;

#[derive(Debug,Serialize, Deserialize)]
struct RemarkableMetadata {
    #[serde(rename = "createdTime")]
    created_time: String,
    #[serde(rename = "lastModified")]
    last_modified: String,
    #[serde(rename = "lastOpened")]
    last_opened: String,
    #[serde(rename = "lastOpenedPage")]
    last_opened_page: u32,
    #[serde(rename = "visibleName")]
    visible_name: String,
    parent: String,
    pinned: bool,
}

#[derive(Debug)]
struct FileWithHash { 
    path: PathBuf,
    hash: Option<String>,
}
#[derive(Debug)]
struct RemarkableObj {
    uuid: String,
    content_hash: String,
    metadata_path: PathBuf,
    metadata: RemarkableMetadata,
    file_paths: Vec<FileWithHash>,
}

#[derive(Parser)]
struct Cli { 
    #[clap(default_value = "/home/user/Documents/remarkable-filesync-rust/remarkable-library/")]
    path_to_library: PathBuf,
}


impl FileWithHash { 
    fn from(path: PathBuf) -> Self {
        let hash: Option<String> = match FileWithHash::generate_file_hash(&path) {
            Ok(t) => Some(t),
            // Manage other error kinds. Right now this is just silent.
            Err(_) => None,
    
        };
        FileWithHash {
            path: path,
            hash: hash
        }
    }
    fn generate_file_hash(path: &PathBuf) -> Result<String, std::io::Error> { 
        let mut file_hash = Sha256::new();
        let file_contents = std::fs::read(path)?;
        file_hash.update(file_contents);
        Ok(format!("{:X}",file_hash.finalize()))
    }

}

impl RemarkableObj {
    fn set_file_extension(original_path: &PathBuf, extension: &str) -> PathBuf {
        let mut temp_path = original_path.clone();
        temp_path.set_extension(extension);
        temp_path
    }
    // The input for this function should be a vector of every file that's 'connected' to this.
    fn new(file_paths: Vec<PathBuf>) -> Self { 
        let mut metadata_path: PathBuf = Default::default();
        let mut uuid: String = Default::default();
        let mut files: Vec<FileWithHash> = Default::default();
        let mut entire_hash = Sha256::new();
        
        for path in file_paths {
            match path.extension() { Some(t) => (), None => continue, }
            if path.extension().unwrap().to_str().unwrap() == "metadata" { 
                println!("{:?}",&path);
                uuid = String::from(RemarkableObj::set_file_extension(&path, "")
                                                                        .file_name()
                                                                        .unwrap()
                                                                        .to_str()
                                                                        .unwrap());        
                metadata_path = path;
                continue;
            }
            let file_with_hash = FileWithHash::from(path);
            match &file_with_hash.hash {
                Some(hash) => entire_hash.update(hash),
                None       => (),
            }
            files.push(file_with_hash);
        };
        let hash_text = format!("{:X}", entire_hash.finalize());
        let metadata_class: RemarkableMetadata = match std::fs::read_to_string(&metadata_path) {
            Ok(data) => serde_json::from_str(data.as_str()).unwrap(),
            Err(e) => panic!("{e}"),
        }; 
        RemarkableObj { 
            uuid: uuid,
            content_hash: hash_text,
            metadata_path: metadata_path,
            metadata: metadata_class,
            file_paths: files,
        }
    }
}

fn group_directory_contents(path: &PathBuf) -> Result<Vec<(String, Vec<PathBuf>)>, std::io::Error> {
    // Let's fetch the contents of the folder and return it as a Vec of Vecs.
    let entire_content = std::fs::read_dir(path)?;
    let paths: Vec<PathBuf> = entire_content.into_iter().map(|x| x.unwrap().path()).collect();
    let groups: Vec<(String, Vec<PathBuf>)> = paths
        .into_iter()
        .group_by(|path| path
                        .file_stem()
                        .unwrap_or_default()
                        .to_str()
                        .unwrap_or_default()
                        .to_owned())
        .into_iter()
        .map(|(prefix, group)| (prefix, group.collect()))
        .collect();
    Ok(groups)
}


fn main() {
    let args = Cli::parse();
    
    //let path = "/home/user/Documents/remarkable-filesync-rust/remarkable-library/";
    let x = group_directory_contents(&args.path_to_library);
    dbg!(&x);
    for (key, list_of_files) in x.unwrap() { 
        let y = RemarkableObj::new(list_of_files);
        dbg!(y);

    }
}
