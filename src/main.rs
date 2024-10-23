use std::{
    io::{Read, Result},
    net::TcpListener,
    os::fd::AsRawFd,
};

use ffi::{
    poll::Poll,
    syscalls::{EpollEvent, EPOLLET, EPOLLIN, EPOLLOUT},
};

mod ffi;

fn main() -> Result<()> {
    let poll = Poll::new()?;
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    listener.set_nonblocking(true)?;
    let listener_fd = listener.as_raw_fd();

    poll.register(listener_fd, 1024, EPOLLOUT | EPOLLET | EPOLLIN)?;

    let mut events: Vec<EpollEvent> = Vec::with_capacity(8);

    loop {
        println!("polling!");
        // wait for events
        poll.poll(None, &mut events)?;

        for event in &events {
            match event.token() {
                1024 => match listener.accept() {
                    Ok((mut stream, _)) => {
                        let mut buf = String::new();
                        stream.read_to_string(&mut buf)?;
                        println!("{}", buf);
                    }
                    Err(e) => eprintln!("ERROR: {}", e),
                },
                token => {
                    println!("da ufck, coe: {}", token)
                }
            }
        }
    }
}
