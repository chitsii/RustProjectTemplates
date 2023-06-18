### Usage
```bash
docker build . -t rust_server 
docker run -p 3000:3000 -t rust_server
curl -i http://localhost:3000
    # HTTP/1.1 200 OK
    # content-type: text/html; charset=utf-8
    # content-length: 11
    # date: Sun, 18 Jun 2023 04:28:51 GMT
    # <h1>OK</h1>⏎    
```