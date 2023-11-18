
use std::{thread, time::Duration, sync::{Arc, Mutex}};

use rocket::form::Form;

use crate::database;
use crate::database::message::{NewMessage, Message};

lazy_static! {
    static ref MESSAGE_QUEUE: Arc<Mutex<Vec<NewMessage>>> = Arc::new(Mutex::new(Vec::new()));
}

#[tauri::command]
pub fn start_webserver(window: tauri::Window) {

    thread::spawn(rocket);
    thread::spawn(move || message_queue_processor(window));

}

fn message_queue_processor(window: tauri::Window) {

    let message_queue = Arc::clone(&MESSAGE_QUEUE);
    loop {

        let messages: Vec<NewMessage> = message_queue.lock().unwrap().drain(..).collect();

        Message::batch_insert(&mut database::connection().unwrap(), messages.clone()).unwrap();

        if messages.len() > 0 {
            window.emit("webhook_message", messages).unwrap();
        }

        thread::sleep(Duration::from_millis(10));

    }

}

#[post("/message", data = "<message>")]
fn message(message: Form<NewMessage>) {

    println!("Received message: {:?}", message);

    let message_queue = Arc::clone(&MESSAGE_QUEUE);
    message_queue.lock().unwrap().push(message.into_inner());

}

pub async fn rocket() {

    rocket::build()
        .mount("/", routes![message])
        .launch()
        .await
        .unwrap();

}