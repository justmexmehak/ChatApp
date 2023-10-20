pub mod user;
pub mod ui;
pub mod user_input;
pub mod room;
pub mod handle_client_request;
pub mod handle_server_response;

use std::{env, net::{TcpListener, TcpStream}, thread, io::{Read, Write}, sync::{Arc, Mutex}, str};
use std::collections::HashMap;
use std::error::Error;


use crate::handle_client_request::RequestType;

use bincode::{deserialize, serialize};

extern crate serde;
#[macro_use]
extern crate serde_derive;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        user::user_tcp();
    }
    else {
        // server 

        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
        println!("Server is listening on 127.0.0.1:7878");

        let active_clients: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(vec![]));

        let rooms: Arc<Mutex<HashMap<String, room::Room>>> = Arc::new(Mutex::new(HashMap::with_capacity(10)));

        for stream in listener.incoming() {
            let stream = stream.expect("Failed to accept a connection");

            let active_clients = Arc::clone(&active_clients);
            let rooms = Arc::clone(&rooms);

            active_clients.lock().unwrap().push(stream.try_clone().expect("Failed to clone"));
    
            thread::spawn(move || {
                handle_connection(stream, active_clients, rooms).unwrap();
            });
        }
    }
}

fn handle_connection(mut stream: TcpStream, active_clients: Arc<Mutex<Vec<TcpStream>>>, rooms: Arc<Mutex<HashMap<String, room::Room>>>) -> Result<(), Box<dyn Error>> {
    println!("Incoming connection from: {}", stream.peer_addr().unwrap());
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf).unwrap();
        if bytes_read == 0 { return Ok(()); }
        let send = &buf[..bytes_read];
        // let req = send;
        let req: RequestType = deserialize(&send).expect("Deserialization failed!");
        println!("{:?}", req);
        let response = handle_client_request::handle_request(req, rooms.lock().unwrap(), &stream, active_clients.lock().unwrap())?;
        println!("{:?}", response);

        let response = serialize(&response).expect("Serialization Failed");
        let response = format!("{}\n", str::from_utf8(&response).unwrap());
        stream.write(&response.as_bytes())?;

        // let text = std::str::from_utf8(&send).expect("did not get");
        // match &text[0..1] {
        //     "1" => {
        //         // create room logic
        //         let mut rooms = rooms.lock().unwrap();
        //         let room_name = "hello";
        //         if rooms.contains_key(room_name) {
        //             stream.write("Room already Exists. Try Again".as_bytes()).unwrap();
        //         } else {
        //             rooms.insert("hello", room::Room::new("hello".to_string(), stream.peer_addr().unwrap()));
        //         }
        //         ()
        //     },
        //     "2" => {
        //         let mut rooms = rooms.lock().unwrap();
        //         let room_name = "hello";
        //         let room = rooms.get_mut(room_name).unwrap();
        //         match room.join_room(stream.peer_addr().unwrap()) {
        //             Ok(()) => println!("User joined room {}", room_name),
        //             Err(()) => {
        //                 stream.write("Room is full. Try Again".as_bytes()).unwrap();
        //                 ()
        //             }
        //         };
                
        //     },
        //     _ => {
        //         let message = format!("Broadcasting: {}\n", text);
        //         broadcast(message.as_bytes(), active_clients.lock().unwrap());
        //         let text = std::str::from_utf8(&buf[..bytes_read]).expect("did not get");
        //         println!("{}", text);
        //     }
        // }
    }
}

