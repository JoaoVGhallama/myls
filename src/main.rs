use std::{fs, path::{PathBuf, Path}};
use owo_colors::OwoColorize;
use clap::Parser;
use strum::Display;

#[derive(Debug, Display)]
enum EntryType {
    File,
    Dir
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = "My ls command")]
struct Cli {
    path:Option<PathBuf>
}

#[derive(Debug)]
struct FileEntry {
    name: String,
    e_type: EntryType,
    len_bytes:u64,
    modified: String
}

fn main() {
    let cli = Cli::parse();

    // take the current cli path, if !path, .
    let path = cli.path.unwrap_or(PathBuf::from("."));

    //check if path does exist 
    if let Ok(does_exist) = fs::exists(&path){
        //iterates get_files return, and print every file/Dir on it
        if does_exist {
            for file in get_files(&path){
                println!("{:?}", file.green())
            }
        }else {
    println!("{}", "Path does not exist".red());
        }
    }else {
    println!("{}", "Cannot determine if path exists. ".red());
    }
}

// get files inside path, return an Vector of files
fn get_files(path: &Path) -> Vec<FileEntry> {
    let mut data = Vec::default();
    if let Ok(read_dir) = fs::read_dir(path){
        for entry in read_dir{
            //big mess here, but it works. Clean later
            if let Ok(file) = entry{
                if let Ok(meta)= fs::metadata(&file.path()){
                data.push(FileEntry {
                        name: file.file_name().into_string().unwrap_or("Unknown file name".into()),
                        e_type: if meta.is_dir() {
                            EntryType::Dir
                        } else {
                            EntryType::File
                        },
                        len_bytes: meta.len(),
                        modified: "".to_string(),
                    }
                )
                }
            }
        }
    }
    data
}
