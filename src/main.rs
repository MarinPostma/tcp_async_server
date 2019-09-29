use std::io;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(stream: TcpStream) -> io::Result<()> {
    let mut buf = String::new();
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut writer = BufWriter::new(stream.try_clone()?);
    writer
        .write(
            &"Connection established...\nWelcome to the super calculation server!\nPlease enter a first number: "
                .as_bytes(),
        )?;
    writer.flush()?;

    // convenient closure to get input
    let mut get_input = |writer: &mut BufWriter<TcpStream>| loop {
        buf.clear();
        reader.read_line(&mut buf).unwrap();
        match buf.trim().parse::<i32>() {
            Ok(num) => break num,
            Err(_) => {
                writer
                    .write("\nInvalid input, enter the first number: ".as_bytes())
                    .unwrap();
            }
        }
        writer.flush().unwrap();
    };

    let first = get_input(&mut writer);

    writer.write("\nEnter the second number: ".as_bytes())?;
    writer.flush()?;

    let second = get_input(&mut writer);
    writer
        .write(format!("The sum of {} and {} is: {}", first, second, first + second).as_bytes())?;
    Ok(())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:5555")?;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client");
                thread::spawn(move || {
                    handle_client(stream).unwrap();
                });
            }
            Err(_) => println!("there was an error"),
        }
    }
    Ok(())
}
