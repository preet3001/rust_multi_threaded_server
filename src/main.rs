use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread::{self},
    time::Duration,
};

use single_threaded_server::ThreadPool;

fn main() -> std::io::Result<()> {
    let listner = TcpListener::bind("127.0.0.1:7676")?;
    let pool = ThreadPool::new(4);
    for stream in listner.incoming() {
        pool.execute(|| handle_client(stream.expect("read stream")).expect("not handling client"));
    }

    Ok(())
}

fn handle_client(mut steam: TcpStream) -> std::io::Result<()> {
    let buf_read = BufReader::new(&mut steam);
    let line = buf_read.lines().next();
    if line.is_some() {
        let request = line.unwrap()?;

        let (status_line, file) = match &request[..] {
            "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
            "GET /sleep HTTP/1.1" => {
                thread::sleep(Duration::from_secs(5));
                ("HTTP/1.1 200 OK", "hello.html")
            }
            _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
        };
        let content = fs::read_to_string(file)?;
        let length = content.len();
        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");
        let _ = steam.write_all(response.as_bytes());
    }
    Ok(())
}
