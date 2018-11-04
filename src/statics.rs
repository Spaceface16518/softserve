pub const PORT: &str = "127.0.0.1:8080";

pub const DEF_RESPONSE: &str = "<!DOCTYPE html>
<html lang=\"en\">
  <head>
    <meta charset=\"utf-8\">
    <title>Hello!</title>
  </head>
  <body>
    <h1>Hello!</h1>
    <p>Hi from Rust</p>
  </body>
</html>";

pub const ERROR_404_RESPONSE: &str = "<!DOCTYPE html>
<html lang=\"en\">
  <head>
    <meta charset=\"utf-8\">
    <title>File Not Found</title>
  </head>
  <body>
    <h1>ERROR 404</h1>
    <p>Oops! Sorry, I don't know what you're asking for.</p>
  </body>
</html>";

pub const GET_PREFIX: &str = "GET / HTTP/1.1\r\n";

pub const STATUS_LINE_200: &str = "HTTP/1.1 200 OK\r\n\r\n";

pub const STATUS_LINE_404: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";

pub const MAX_THREADS: usize = 4;
