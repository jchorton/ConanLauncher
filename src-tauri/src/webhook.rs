
use iron::prelude::*;
use iron::status;
use router::Router;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use bodyparser;

use crate::database;
use crate::database::message::{NewMessage, Message};

lazy_static! {
    static ref MESSAGE_QUEUE: Arc<Mutex<Vec<NewMessage>>> = Arc::new(Mutex::new(Vec::new()));
}

#[tauri::command]
pub fn start_webserver(window: tauri::Window) {

    thread::spawn(move || message_queue_processor(window));
    thread::spawn(|| {
        let mut router = Router::new(); 
        router.post("/", post_message, "post");
        Iron::new(router).http("localhost:3000").unwrap();
    });

}

fn message_queue_processor(window: tauri::Window) {

    let message_queue = Arc::clone(&MESSAGE_QUEUE);
    loop {
        let messages: Vec<NewMessage> = message_queue.lock().unwrap().drain(..).collect();

        Message::batch_insert(&mut database::connection().unwrap(), messages.clone()).unwrap();

        if !messages.is_empty() {
            window.emit("webhook_message", messages).unwrap();
        }

        thread::sleep(Duration::from_millis(10));
    }

}

fn post_message(req: &mut Request) -> IronResult<Response> {

    println!("Received message");
    println!("{:?}", req);

    let message_queue = Arc::clone(&MESSAGE_QUEUE);
    let message_body = req.get::<bodyparser::Struct<NewMessage>>();

    match message_body {
        Ok(Some(message)) => {
            message_queue.lock().unwrap().push(message);
            Ok(Response::with((status::Ok, "Message received")))
        }
        _ => Ok(Response::with((status::BadRequest, "Invalid message"))),
    }

}
