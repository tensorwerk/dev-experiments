# Overview

The file transfer utility is broken into a client and a server.

The current implementation is the dumbest and simplest possible.

```sh
# to build
./build
# to run the server
./run_server.sh
# to run the client
./run_client.sh file
```

See the above scripts for more usage information.

The server will run and serve a single TCP connection. The client reads a file,
hashes it, and transfers its hash and contents to the server. The server then
compares the expected and actual content hash.

At present, the TCP stream is buffered on both ends.

The CHUNK_SIZE env var on the client (partially...) controls how fast file
contents are written to the raw socket.

## TODO

- Make the server use an asynchronous implementation that handles simultaneous
    clients.
- Add profiling/benchmarking.
- Expose TCP_NODELAY and IP_TTL
- See about using
- Figure out if chunk size negotiation makes sense.
