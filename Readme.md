# HTTP-to-IRC bridge
This is a simple IRC bot that exposes an HTTP endpoint for sending messages to channels and users. Incoming messages are ignored.

Written in Rust (half as an experiment and probably full of bugs), should compile with 1.0.0

Needs test, documentation and lots of proper error handling.

Example (listening to 36005):
```bash
# This will send a private message
curl -d "This is a test" "http://localhost:36005/private/marekventur"

# This will post to channel "some-channel"
curl -d "This is a test" "http://localhost:36005/some-channel"
```