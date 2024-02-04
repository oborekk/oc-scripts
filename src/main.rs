pub mod docker;

use crate::docker::docker_image;
use bollard::container::LogOutput;
use futures_util::{Stream, TryStreamExt};
use rocket::fs::FileServer;
use rocket::response::stream::{Event, EventStream};
use rocket::tokio::time::{self, Duration};
use rocket::{launch, routes};
#[macro_use]
extern crate rocket;

#[get("/events")]
fn stream() -> EventStream![] {
    EventStream! {
        // let mut interval = time::interval(Duration::from_secs(1));
        // let mut count = 0;
        // loop {
        //     let content = format!("<h2>ping  {}</h2>", count);
        //     yield Event::data(content);
        //     interval.tick().await;
        //     count += 1;
        // }

        let mut logs = docker_image("e052fcf8b981").await.unwrap();
        // loop {
            while let Ok(Some(output)) = &logs.try_next().await {
                match output {
                    LogOutput::StdOut { message } => yield Event::data(String::from_utf8(message.to_vec()).unwrap()),
                    LogOutput::StdErr { message } => yield Event::data(String::from_utf8(message.to_vec()).unwrap()),
                    _ => (),
                }
            }
        // }
    }
}

#[get("/start")]
fn start() -> &'static str {
    "<div hx-ext=\"sse\" sse-connect=\"/events\" sse-swap=\"message\">"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![stream, start])
        .mount("/", FileServer::from("src/static"))
}
