use std::{
    io::{prelude::*, Error, ErrorKind},
    net::TcpStream,
    thread,
};

use regex::Regex;

fn pip(mut from: TcpStream, mut to: TcpStream) -> Result<(), Error> {
    let mut buffer = [0u8; 1024];
    let mut n = from.read(&mut buffer[..])?;
    while n > 0 {
        to.write_all(&mut buffer[..n])?;
        n = from.read(&mut buffer[..])?;
    }
    Ok(())
}
pub fn handle_connection(mut stream: TcpStream) -> Result<(), Error> {
    let local_stream_read = stream.try_clone()?;
    let mut local_stream_write = stream.try_clone()?;

    let mut head = [0u8; 2048];
    let n = stream.read(&mut head[..])?;

    let head_str =
        std::str::from_utf8(&head[..n]).map_err(|x| Error::new(ErrorKind::Interrupted, x))?;
    let reg = Regex::new(r"(CONNECT|Host:) ([^ :\r\n]+)(?::(\d+))?").unwrap();
    if let Some(caps) = reg.captures(head_str) {
        let host = &caps[2];
        let port = caps.get(3).map_or("80", |m| m.as_str());
        println!("{} {}", host, port);
        let dst_addr = format!("{}:{}", host, port);
        let remote_stream_read = TcpStream::connect(dst_addr).unwrap();
        let mut remote_stream_write = remote_stream_read.try_clone().unwrap();

        if head_str.starts_with("CONNECT") {
            local_stream_write
                .write_all("HTTP/1.1 200 Connection Established\r\n\r\n".as_bytes())?;
        } else {
            remote_stream_write.write_all(&head[..n])?;
        }

        thread::spawn(move || {
            if let Err(_) = pip(remote_stream_read, local_stream_write) {
                ()
            }
        });
        thread::spawn(move || {
            if let Err(_) = pip(local_stream_read, remote_stream_write) {
                ()
            }
        });
        // Ok(())
    } else {
        // let err_msg = format!("TCP 头部不是HTTPS CONNECT请求 或者 HTTP消息:\r\n {}", head_str);
        // Err(Error::new(
        //     ErrorKind::Other,
        //     err_msg,
        // ))
    }
    Ok(())
    // println!("Request: {:#?}", http_request);
}
