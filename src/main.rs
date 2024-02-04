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

        let logs = docker_image("e052fcf8b981").unwrap();
        // loop {
            yield Event::data("wip");
            while let Ok(Some(output)) = &logs.try_next().await {
                match output {
                    LogOutput::StdOut { message } => yield Event::data(message.into_string()),
                    LogOutput::StdErr { message } => yield Event::data(message),
                    _ => (),
                }
            }
        // }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![stream])
        .mount("/", FileServer::from("src/static"))
}
