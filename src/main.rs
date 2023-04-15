use std::{
    io::{prelude::*, BufReader, Result},
    net::{TcpListener, TcpStream}
};

fn main() {
    let port = "7878";
    // we use unwrap here because tcp bind can fail
    let listener = TcpListener::bind(String::from("127.0.0.1:") + port).unwrap();
    println!("server started at port {}", port);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream).unwrap();
    }
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

/*
I'll explain some reason why the buf reader can't be put outside
of the loop. So stream is a 2-way communication thingy.

When we make a buffered reader, we can't use stream to write anymore
because there is mutable reference to it from buf_reader.

When we put it in a loop, it works because buf_reader no longer in scope
when we try to do stream.write_all. If we somehow need to read after
write in the loop statement, I think it will fail!
*/
fn handle_connection(mut stream: TcpStream) -> Result<()> {

    loop {
        // why can't we put this outside the loop?
        let mut buf_reader = BufReader::new(&mut stream);
        
        let mut line = String::new();
        buf_reader.read_line(&mut line)?;

        stream.write_all(line.as_bytes()).unwrap();

        trim_newline(&mut line);
        println!("line: {}", line);

        if line == "quit" {
            break;
        }
    }

    Result::Ok(())
}

