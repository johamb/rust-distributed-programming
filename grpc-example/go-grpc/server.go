package main

import (
	"fmt"
	"log"
	"net"
	"errors"

	"./build/gen"

	"google.golang.org/grpc"
	"golang.org/x/net/context"
)

type Server struct {
	notes []notes.Note
}

func (s *Server) GetNoteByTitle(ctx context.Context, title *notes.Title) (*notes.Note, error) {

	for _, note := range notes {
		if (note.title == title) {
			return note, nil
		}
	}

	return nil, errors.New("note not found")
}

func main() {

	lis, err := net.Listen("tcp", fmt.Sprintf(":%d", 9000))
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}

	s := Server{}

	grpcServer := grpc.NewServer()

	notes.RegisterNoticeboardServer(grpcServer, &s)

	if err := grpcServer.Serve(lis); err != nil {
		log.Fatalf("failed to serve: %s", err)
	}
}