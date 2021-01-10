pub mod notes {
    tonic::include_proto!("notes");
}

use std::error::Error;

use tonic::transport::{Endpoint, Channel};
use tonic::Request;

use notes::noticeboard_client::NoticeboardClient;
use notes::{Author, Title};

async fn get_notes_by_author(
    client: &mut NoticeboardClient<Channel>,
    author_mail: &str
) -> Result<(), Box<dyn Error>> {
    let author = Author {
        nickname: "".to_string(),
        mail: author_mail.to_string(),
    };

    let mut stream = client
        .list_notes_by_author(Request::new(author))
        .await?
        .into_inner();

    while let Some(note) = stream.message().await? {
        println!("Note received: {:?}", note);
    }
    println!("finished");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = NoticeboardClient::connect(Endpoint::from_static("tcp://127.0.0.1:9000")).await?;

    let response = client
        .get_note_by_title(Request::new(Title {
            title: "Hello".to_string(),
        }))
        .await?;

    println!("Note received: {:?}", response.get_ref());

    println!("\n Streaming notes from the server:");
    get_notes_by_author(&mut client, "hans@gmail.com").await?;

    Ok(())
}
