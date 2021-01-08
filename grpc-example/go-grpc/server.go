package main

import (
	"fmt"
	"log"
	"net"
	"errors"
	"time"

	"./build/gen"

	"google.golang.org/grpc"
	"golang.org/x/net/context"
)

type Server struct {
	notes []notes.Note
}

func (s *Server) GetNoteByTitle(ctx context.Context, title *notes.Title) (*notes.Note, error) {

	for _, note := range s.notes {
		if (note.Title == title.Title) {
			return &note, nil
		}
	}

	return nil, errors.New("note not found")
}

func (s *Server) ListNotesByAuthor(author *notes.Author, srv notes.Noticeboard_ListNotesByAuthorServer) error {
	
	for _, note := range s.notes {
		if (note.Author.Mail == author.Mail) {
			srv.Send(&note)
		}
		// uncomment this to see that streaming the notes actually works
		// time.Sleep(2 * time.Second)
	}

	return nil

}

func main() {

	lis, err := net.Listen("tcp", fmt.Sprintf(":%d", 9000))
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}

	s := Server{
		notes: []notes.Note{
            notes.Note {
                Title: "Hello",
                Content: "This note says hello.",
                Author: &notes.Author {
                    Nickname: "Hans",
                    Mail: "hans@gmail.com",
                },
            },
            notes.Note {
                Title: "Goodbye",
                Content: "This note says goodbye.",
                Author: &notes.Author {
                    Nickname: "Hans",
                    Mail: "hans@gmail.com",
                },
            },
            notes.Note {
                Title: "What up",
                Content: "This note says what up.",
                Author: &notes.Author {
                    Nickname: "Lisa",
                    Mail: "lisa@gmail.com",
                },
            },
        },
	}

	grpcServer := grpc.NewServer()

	notes.RegisterNoticeboardServer(grpcServer, &s)

	if err := grpcServer.Serve(lis); err != nil {
		log.Fatalf("failed to serve: %s", err)
	}
}