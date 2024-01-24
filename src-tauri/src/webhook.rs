
use iron::prelude::*;
use iron::status;
use router::Router;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use urlencoded::UrlEncodedQuery;

use crate::database;
use crate::database::message::{NewMessage, Message};

lazy_static! {
    static ref MESSAGE_QUEUE: Arc<Mutex<Vec<NewMessage>>> = Arc::new(Mutex::new(Vec::new()));
    static ref TEXT_USER: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
}


#[tauri::command]
pub fn start_webserver(window: tauri::Window) {

    thread::spawn(move || message_queue_processor(window));
    thread::spawn(|| {
        let mut router = Router::new(); 
        router.post("/", post_message, "post");
        router.get("/", post_message, "get");
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

    let query = match req.get_ref::<UrlEncodedQuery>() {
        Ok(query) => query,
        Err(_) => return Ok(Response::with((status::BadRequest, "Invalid query"))),
    };

    let message = query.get("message").and_then(|v| v.first()).map_or("", |v| v);
    let sender = query.get("sender").and_then(|v| v.first()).map_or("", |v| v);

    let new_message = NewMessage {
        sender: sender.to_string(),
        message: message.to_string(),
    };

    let message_queue = Arc::clone(&MESSAGE_QUEUE);
    message_queue.lock().unwrap().push(new_message);

    Ok(Response::with((status::Ok, "Message received")))
    
}
