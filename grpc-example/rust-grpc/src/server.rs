use std::sync::Arc;
use core::time::Duration;

use tokio::sync::mpsc;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

use notes::noticeboard_server::{Noticeboard, NoticeboardServer};
use notes::{Author, Note, Title};

pub mod notes {
    // You need to pass the name of the package declared in the .proto file here. In this case: notes.
    tonic::include_proto!("notes");
}
 
// #[...] is an attribute. derive(Debug) causes the compiler to auto-generate an implementation for this trait
#[derive(Debug)]
pub struct NoticeboardService {
    notes: Arc<Vec<Note>>,
}

#[tonic::async_trait]
impl Noticeboard for NoticeboardService {

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
        let (mut sender, receiver) = mpsc::channel(100);
        let notes = self.notes.clone();

        tokio::spawn(async move {
            for note in &notes[..] {
                match &note.author {
                    Some(a) => {
                        if a.mail == request.get_ref().mail {
                            sender.send(Ok(note.clone())).await.unwrap();
                        }
                    }
                    _ => (),
                }
            }
        });

        Ok(Response::new(receiver))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:9000".parse().unwrap();

    let noticeboard = NoticeboardService {
        notes: Arc::new(vec![
            Note {
                title: "Hello".to_string(),
                content: "This note says hello.".to_string(),
                author: Some(Author {
                    nickname: "Hans".to_string(),
                    mail: "hans@gmail.com".to_string(),
                }),
            },
            Note {
                title: "Goodbye".to_string(),
                content: "This note says goodbye.".to_string(),
                author: Some(Author {
                    nickname: "Hans".to_string(),
                    mail: "hans@gmail.com".to_string(),
                }),
            },
            Note {
                title: "What up".to_string(),
                content: "This note says what up.".to_string(),
                author: Some(Author {
                    nickname: "Lisa".to_string(),
                    mail: "lisa@gmail.com".to_string(),
                }),
            },
        ]),
    };

    let svc = NoticeboardServer::new(noticeboard);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
