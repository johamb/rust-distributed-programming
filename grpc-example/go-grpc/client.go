package main

import (
	"context"
	"io"
	"log"

	noticeboard "./build/gen"

	"google.golang.org/grpc"
)

func main() {
	// connect to the server
	srv, err := grpc.Dial(":9000", grpc.WithInsecure())
	if err != nil {
		log.Fatalf("can not connect with server %v", err)
	}

	client := noticeboard.NewNoticeboardClient(srv)
	author := &noticeboard.Author{Nickname: "", Mail: "hans@gmail.com"}
	// open the stream
	stream, err := client.ListNotesByAuthor(context.Background(), author)
	if err != nil {
		log.Fatalf("open stream error %v", err)
	}

	// this channel is used to tell the main thred when we are done
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
			log.Printf("Resp received: %s", resp.Content)
		}
	}()

	<-ch //wait until
	log.Printf("finished")
}