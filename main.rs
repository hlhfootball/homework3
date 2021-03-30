use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;

fn handle_client(mut stream: TcpStream) -> Result<(), Error>{
    let mut buf = [0; 512]; //instance a buff 
    for _ in 0..1000 {
        let bytes_read = stream.read(&mut buf)?;    //read data to buf from stream,if read error return error
        if bytes_read == 0 {
            return Ok(());  //if has no data to read any more,return ok
        }
        
        for i in 0..bytes_read {
            println!("{}",buf[i]);
        }

        stream.write(&buf[..bytes_read])?;  //write the data back
        thread::sleep(time::Duration::from_secs(1 as u64)); //sleep 1 s
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;  //instance a listener,if bind error return error
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();   //instance a vector for thread handle

    for stream in listener.incoming() {
        let stream = stream.expect("failed!"); //if stream return failed
        let handle = thread::spawn(move || {
            handle_client(stream)
        .unwrap_or_else(|error| eprintln!("{:?}", error));
        }); //instance a thread to deal with stream,if error return

        thread_vec.push(handle);    //push the thread handle to the vector
    }

    for handle in thread_vec {
        handle.join().unwrap(); //wait for finish thread
    }

    Ok(())  //return ok
}
