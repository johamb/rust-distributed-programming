package main

import (
	"fmt"
	"log"
	"net"
	"errors"
	"context"
	//"time"

	noticeboard "./build/gen"

	"google.golang.org/grpc"
)

type NoticeboardService struct {
	noticeboard []noticeboard.Note
}

const port = 9000

func (s *NoticeboardService) GetNoteByTitle(ctx context.Context, title *noticeboard.Title) (*noticeboard.Note, error) {

	for _, note := range s.noticeboard {
		if (note.Title == title.Title) {
			return &note, nil
		}
	}

	return nil, errors.New("note not found")
}

func (s *NoticeboardService) ListNotesByAuthor(author *noticeboard.Author, srv noticeboard.Noticeboard_ListNotesByAuthorServer) error {
	
	for _, note := range s.noticeboard {
		if (note.Author.Mail == author.Mail) {
			srv.Send(&note)
		}
		// uncomment this to see that streaming the noticeboard actually works
		// time.Sleep(2 * time.Second)
	}

	return nil

}

func main() {

	lis, err := net.Listen("tcp", fmt.Sprintf(":%d", port))
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}

	s := NoticeboardService{
		noticeboard: []noticeboard.Note{
            noticeboard.Note {
                Title: "Hello",
                Content: "This note says hello.",
                Author: &noticeboard.Author {
                    Nickname: "Hans",
                    Mail: "hans@gmail.com",
                },
            },
            noticeboard.Note {
                Title: "Goodbye",
                Content: "This note says goodbye.",
                Author: &noticeboard.Author {
                    Nickname: "Hans",
                    Mail: "hans@gmail.com",
                },
            },
            noticeboard.Note {
                Title: "What up",
                Content: "This note says what up.",
                Author: &noticeboard.Author {
                    Nickname: "Lisa",
                    Mail: "lisa@gmail.com",
                },
            },
        },
	}

	grpcServer := grpc.NewServer()

	noticeboard.RegisterNoticeboardServer(grpcServer, &s)

	if err := grpcServer.Serve(lis); err != nil {
		log.Fatalf("failed to serve: %s", err)
	}
}