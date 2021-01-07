use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Instant;

use futures_core::Stream;
use tokio::sync::mpsc;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

use notes::noticeboard_server::{Noticeboard, NoticeboardServer};
use notes::{Note, Author, Title};

pub mod notes {
    // You need to pass the name of the package declared in the .proto file here. In this case: notes.
    tonic::include_proto!("notes");
}

#[derive(Debug)]
struct NoteService;

#[tonic::async_trait]
impl Noticeboard for NoteService {
    async fn get_note_by_title(&self, _request: Request<Title>) -> Result<Response<Note>, Status> {
        unimplemented!()
    }

    type ListNotesByAuthorStream = mpsc::Receiver<Result<Note, Status>>;

    async fn list_notes_by_author(
        &self,
        _request: Request<Author>,
    ) -> Result<Response<Self::ListNotesByAuthorStream>, Status> {
        unimplemented!()
    }

    async fn add_notes(
        &self,
        _request: Request<tonic::Streaming<Note>>,
    ) -> Result<Response<()>, Status> {
        unimplemented!()
    }
}

fn main() {
    println!("Hello, world!");
}

