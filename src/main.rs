pub mod docker;

use crate::docker::docker_image;
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

        docker_image();
        loop {
            yield Event::data("wip");
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![stream])
        .mount("/", FileServer::from("src/static"))
}
