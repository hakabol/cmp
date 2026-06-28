use std::{ffi::OsStr, fs, path::PathBuf, vec};

use serde::Serialize;

const NEEDED_EXT: &[&str] = &[
    "rs",
    "toml",
    "lock",
    "json",
    "yaml",
    "yml",
    "ron",
    "md",
    "txt",
    "sh",
    "bash",
    "nix",
    "py",
    "ipynb"
];

#[derive(Serialize)]
struct Chunk {
    kind: String,
    name: String,
    code: String,
}

#[derive(Serialize)]
struct File {
    path: String,
    chunks: Vec<Chunk>,
}

#[derive(Serialize)]
struct Repo {
    name: String,
    files: Vec<File>,
}

fn main() {
    let path = clone_url("https://github.com/BurntSushi/ripgrep").unwrap();

    let files = walk_dirs(path);

    for file in files{
        println!("{}", file.display());
    }

    fs::remove_dir_all("temp").unwrap();
}

fn clone_url(url: &str) -> anyhow::Result<PathBuf>{
    let repo_name = url
        .split('/')
        .last()
        .unwrap()
        .trim_end_matches(".git");

    let path = PathBuf::from("temp").join(repo_name);

    std::fs::create_dir_all("temp")?;

    git2::Repository::clone(url, &path)?;

    Ok(path)
}

fn walk_dirs(path: PathBuf) -> Vec<PathBuf>{
    let mut files: Vec<PathBuf> = vec![];

    for entry in fs::read_dir(path).unwrap(){
        let entry_idk = entry.unwrap();
        let sub_path = entry_idk.path();

        if sub_path.is_dir(){
            let mut sub_files = walk_dirs(sub_path);

            files.append(&mut sub_files);
        }
        else{
            if let Some(ext) = sub_path.extension().unwrap_or(OsStr::new("")).to_str(){
                if NEEDED_EXT.contains(&ext){
                    files.push(sub_path)
                }
            }
        }
    }

    files
}
