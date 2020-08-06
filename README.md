# readysetlog

Basic echo server; logs all incoming requests.

## Installation / Usage

```bash
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ cargo install readysetlog
$ readysetlog <port>
```

## Notes
This process contains three servers in total - the echo server itself, a HTTP server, and a WS server. 
The latter two are part of an (so far unfinished) attempt to log requests in a web UI in addition to the CLI.
