docker build -t bible-api-rust .
docker run -d -p 8080:8080 bible-api-rust
