# Go gRPC Server Client Example
## Requirements
- [Go](https://golang.org/doc/install)
- [protoc](https://developers.google.com/protocol-buffers/docs/gotutorial#compiling-your-protocol-buffers) (only necessary if you want to generate the source code already included in /build/gen)

## Run Commands
- Server: `go run server.go`
- Client: `go run client.go`
- To generate the source code in /build/gen: `protoc --proto_path=../proto --go_out=plugins=grpc:build/gen ../proto/notes.proto`
