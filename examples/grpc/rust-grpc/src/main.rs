use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Instant;

use futures_core::Stream;
use tokio::sync::mpsc;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

use notes::noticeboard_server::{Noticeboard, NoticeboardServer};
use notes::{Author, Note, Title};

pub mod notes {
    // You need to pass the name of the package declared in the .proto file here. In this case: notes.
    tonic::include_proto!("notes");
}

#[derive(Debug)]
pub struct NoteService {
    notes: Vec<Note>,
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
        request: Request<Author>,
    ) -> Result<Response<Self::ListNotesByAuthorStream>, Status> {
        // TODO: read docs for mpsc::channel
        let (mut tx, rx) = mpsc::channel(4);
        let notes = self.notes.clone();

        tokio::spawn(async move {
            for note in &notes[..] {
                match &note.author {
                    Some(a) => {
                        if a.mail == request.get_ref().mail {
                            tx.send(Ok(note.clone())).await.unwrap();
                        }
                    }
                    _ => (),
                }
            }
        });

        Ok(Response::new(rx))
    }

    // vlt ersetzen durch add_note
    async fn add_notes(
        &self,
        request: Request<tonic::Streaming<Note>>,
    ) -> Result<Response<()>, Status> {
        let mut stream = request.into_inner();

        while let Some(note) = stream.message().await? {
            // let note = note?;
            // let mut noteExists = false;
            // for existingNote in &self.notes[..] {
            //     if existingNote.title == &note.title {
            //         noteExists = true;
            //     }
            // }
            &self.notes.push(note);
        }

        Ok(Response::new(()))
    }
}

fn main() {
    println!("Hello, world!");
}
