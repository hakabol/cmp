use std::{env, ffi::OsStr, fs, path::PathBuf, vec};

use serde::{Serialize, Deserialize};

const NEEDED_EXT: &[&str] = &[
    "rs",
    "toml",
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

#[derive(Serialize, Deserialize)]
struct Chunk {
    code: String,
}

#[derive(Serialize, Deserialize)]
struct File {
    path: String,
    chunks: Vec<Chunk>,
}

#[derive(Serialize, Deserialize)]
struct Repo {
    name: String,
    files: Vec<File>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let repo = &args[1];
    let name = repo.split("/").last().unwrap();
    let path = clone_url(repo).unwrap();

    let files = walk_dirs(path);

    println!("reading repo");

    let repo = to_repo(files, name.to_string());

    println!("repo ready and cloned");

    let text = fs::read_to_string("data.json").unwrap();

    let mut repos: Vec<Repo> = serde_json::from_str(&text).unwrap();
    //let mut repos: Vec<Repo> = vec![];
    repos.push(repo);

    let json = serde_json::to_string_pretty(&repos).unwrap();

    println!("json ready");

    fs::write("data.json", json).unwrap();
    fs::remove_dir_all("temp").unwrap();

    println!("done");

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

fn reader(path: PathBuf) -> Vec<Chunk>{
    let mut paras: Vec<Chunk> = vec![];

    let text = fs::read_to_string(path).unwrap();
    let paras_it = text.split("\n\n");

    for para in paras_it{
        paras.push(Chunk { code: para.to_string() });
    }

    paras
}

fn to_repo(paths: Vec<PathBuf>, repo_name: String) -> Repo{
    let mut files: Vec<File> = vec![];

    for path in paths{
        let s = path.to_string_lossy();
        let name = &s[5..];

        let chunks = reader(path.clone());

        files.push(File { path: name.to_string(), chunks })

    }

    return Repo {name: repo_name, files: files}
}
