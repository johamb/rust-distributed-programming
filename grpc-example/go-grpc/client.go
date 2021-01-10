package main

import (
	"context"
	"io"
	"log"
	"fmt"

	noticeboard "./build/gen"

	"google.golang.org/grpc"
)

const port = 9000

func main() {
	// connect to the server
	ctx := context.Background()
	srv, err := grpc.DialContext(ctx, fmt.Sprintf("localhost:%d", port), grpc.WithInsecure())
	if err != nil {
		log.Fatalf("can not connect with server %v", err)
	}
	log.Printf("state: %v", srv.GetState())

	client := noticeboard.NewNoticeboardClient(srv)

	var title = noticeboard.Title{Title: "What up"} 
	note, err := client.GetNoteByTitle(ctx, &title)
	if err != nil {
		log.Fatalf("Error receiving note: %v", err)
	}
	log.Printf("Note received: %s", note.Content)

	log.Printf("Streaming notes from the server")
	author := &noticeboard.Author{Nickname: "", Mail: "hans@gmail.com"}
	// open the stream
	stream, err := client.ListNotesByAuthor(context.Background(), author)
	if err != nil {
		log.Fatalf("open stream error %v", err)
	}

	// this channel is used to tell the main thread when we are done
	ch := make(chan bool)

	go func() {
		for {
			resp, err := stream.Recv()
			if err == io.EOF {
				ch <- true 
				return
			}
			if err != nil {
				log.Fatalf("cannot receive %v", err)
			}
			log.Printf("Note received: %s", resp.Content)
		}
	}()

	<-ch //wait until
	log.Printf("finished")
}