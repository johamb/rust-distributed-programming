#[derive(Debug)]
struct NoteService;

pub mod notes {
    tonic::include_proto!("notes");
}

fn main() {
    println!("Hello, world!");
}

