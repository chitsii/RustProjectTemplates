### Usage
```bash
docker build . -t rust_server 
docker run -p 3000:3000 -t rust_server
curl http://localhost:3000
    > <h1>Hello, World!</h1>⏎
```