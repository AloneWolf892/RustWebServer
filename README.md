Web Server
Based on the webserver of the rust documentation.

This is just a small project for me to learn more about Rust and the HTTP protocol.

Don't try to use this in a real environment, I have no idea what kinds of security issues this might have.

Features:
- Dynamically scan a website root folder to be able to serve files within it without restarting the server or modifying the code.

- Serving is done via HTTP GET requests and responses, other kinds of requests are not implemented yet