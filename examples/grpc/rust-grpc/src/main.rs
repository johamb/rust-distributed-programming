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
pub struct NoteService {
    notes: Arc<Vec<Note>>,
}

#[tonic::async_trait]
impl Noticeboard for NoteService {
    async fn get_note_by_title(&self, request: Request<Title>) -> Result<Response<Note>, Status> {
        for note in &self.notes[..] {
            if note.title == request.get_ref().title {
                return Ok(Response::new(note.clone()));
            }
        }
        Ok(Response::new(Note::default()))
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

