use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;
use std::io::{self, BufRead, BufReader, Write};

use crate::handle_client_request::RequestType;
use crate::ui;
use crate::user_input;

// use crate::handle_client_request::RequestType;

use bincode::serialize;

pub fn user_tcp() {
    // implement client logic
    let mut stream = TcpStream::connect("127.0.0.1:7878").expect("Failed to connect to the server");

    ui::header();

    ui::category_prompt();
    let choice = user_input::get_category_input();
    println!("{:?}", choice);

    stream.write(&serialize(&choice).unwrap()).unwrap();
    

    loop {
        let mut stream_clone = stream.try_clone().expect("Failed to clone");
        // Spawn a thread to handle user input
        thread::spawn(move || {
            loop {
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read from stdin");
                // stream_clone.write(input.as_bytes()).expect("Failed to write to server");
                let message = RequestType::SendMessage(input);
                stream_clone.write(&serialize(&message).unwrap()).unwrap();
            }
        });
        


        let mut reader = BufReader::new(&stream);
        let mut buffer = String::new();
        reader.read_line(&mut buffer).expect("Could not read into buffer");
        print!("{}", buffer);
    }
}