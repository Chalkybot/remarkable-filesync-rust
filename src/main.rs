use std::path::PathBuf;
use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};


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

impl FileWithHash { 
    fn from(path: PathBuf) -> Self {
        let hash: Option<String> = match FileWithHash::generate_file_hash(&path) {
            Ok(t) => Some(t),
            // Manage other error kinds. Right now this is just silent.
            Err(ref e) => None,
    
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
    // This is better for my use than using the PatBuf.set_extension as that requires a temporary mutable.
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
            if path.extension().unwrap().to_str().unwrap() == "metadata" { 
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
            Ok(Data) => serde_json::from_str(Data.as_str()).unwrap(),
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
fn list_files(vec: &mut Vec<PathBuf>, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if std::fs::metadata(&path)?.is_dir() {
        let paths = std::fs::read_dir(&path)?;
        for path_result in paths {
            let full_path = path_result?.path();
            if std::fs::metadata(&full_path)?.is_dir() {
                list_files(vec, &full_path)?
            } else {
                vec.push(full_path);
            }
        }
    }
    Ok(())
}


fn fetch_library(path: &str)  { 
    let path = PathBuf::from(path);
    let mut files = Vec::new();
    list_files(&mut files, &path);
}


fn main() {
/*    let paths = vec![
                PathBuf::from("remarkable-library/f1f5c131-ec35-4e79-8e23-2092412aa75d/fc818769-7c76-4db2-8f48-15b04db4b8e9.rm"),
                PathBuf::from("remarkable-library/f1f5c131-ec35-4e79-8e23-2092412aa75d/63f9a996-b6b6-4f92-b11d-54dc30058e3c.rm"),
                PathBuf::from("remarkable-library/f1f5c131-ec35-4e79-8e23-2092412aa75d/3b7cf0cd-6d7f-427d-a9af-65d01f5098bc.rm"),
                PathBuf::from("remarkable-library/f1f5c131-ec35-4e79-8e23-2092412aa75d.content"),
                PathBuf::from("remarkable-library/f1f5c131-ec35-4e79-8e23-2092412aa75d.epub"),
                PathBuf::from("remarkable-library/f1f5c131-ec35-4e79-8e23-2092412aa75d.epubindex"),
                PathBuf::from("remarkable-library/f1f5c131-ec35-4e79-8e23-2092412aa75d.metadata"),
                PathBuf::from("remarkable-library/f1f5c131-ec35-4e79-8e23-2092412aa75d.pagedata"),
                PathBuf::from("remarkable-library/f1f5c131-ec35-4e79-8e23-2092412aa75d.pdf"),
                PathBuf::from("remarkable-library/f1f5c131-ec35-4e79-8e23-2092412aa75d.thumbnails"),
                PathBuf::from("remarkable-library/f1f5c131-ec35-4e79-8e23-2092412aa75d.thumbnails/3b7cf0cd-6d7f-427d-a9af-65d01f5098bc.png"),
                PathBuf::from("remarkable-library/f1f5c131-ec35-4e79-8e23-2092412aa75d.thumbnails/fc818769-7c76-4db2-8f48-15b04db4b8e9.png"),
                PathBuf::from("remarkable-library/f1f5c131-ec35-4e79-8e23-2092412aa75d.thumbnails/4484edc4-1629-4692-ac28-05d038b70730.png"),
                PathBuf::from("remarkable-library/f1f5c131-ec35-4e79-8e23-2092412aa75d.thumbnails/63f9a996-b6b6-4f92-b11d-54dc30058e3c.png")]; */
    let path = "/home/user/Documents/remarkable-filesync-rust/remarkable-library";
     fetch_library(path);

}
