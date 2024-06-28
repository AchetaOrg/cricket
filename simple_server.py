from http.server import BaseHTTPRequestHandler, HTTPServer

class SimpleHandler(BaseHTTPRequestHandler):
    def _send_response(self):
        self.send_response(200)
        self.send_header('Content-type', 'text/html')
        self.end_headers()
        self.wfile.write(b'OK')

    def do_GET(self):
        print(f"Received GET request for {self.path}")
        self._dump_headers()
        self._send_response()

    def do_POST(self):
        print(f"Received POST request for {self.path}")
        self._dump_headers()
        content_length = int(self.headers.get('Content-Length', 0))
        body = self.rfile.read(content_length)
        print(f"Body: {body.decode('utf-8')}")
        self._send_response()

    def _dump_headers(self):
        print("Headers:")
        for header, value in self.headers.items():
            print(f"{header}: {value}")

def run(server_class=HTTPServer, handler_class=SimpleHandler, port=8000):
    server_address = ('', port)
    httpd = server_class(server_address, handler_class)
    print(f'Starting httpd server on port {port}')
    httpd.serve_forever()

if __name__ == "__main__":
    run()
