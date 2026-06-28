use serde::Serialize;

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
    println!("Hello, world!");
}
