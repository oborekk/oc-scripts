pub mod docker;

use crate::docker::docker_setup;
use bollard::container::LogOutput;
use docker::{docker_logs, docker_remove};
use futures_util::TryStreamExt;
use rocket::fs::FileServer;
use rocket::{launch, routes};
#[macro_use]
extern crate rocket;

#[post("/start/<option>")]
async fn start(option: &str) -> String {
    // "<div hx-ext=\"sse\" sse-connect=\"/events\" sse-swap=\"message\" sse-target=\"#celsius\">"
    let id = docker_setup(option).await;
    match id {
        Ok(v) => format!(
            // "<div hx-trigger=\"done\" hx-get=\"/logs\" hx-swap=\"outerHTML\" hx-target=\"this\">
            //     <p
            //         hx-get=\"/logs/{v}\"
            //         hx-trigger=\"every 500ms\"
            //         hx-target=\"this\"
            //         hx-swap=\"innerHTML\">
            //     Logs will start here</p>
            // </div>"
            "<div hx-swap=\"outerHTML\" hx-get=\"/logs/{v}\" hx-trigger=\"every 1s\"></div>"
        ),
        Err(e) => {
            format!("<div>Error! {e}</div>")
        }
    }
}

#[get("/logs/<id>")]
async fn logs(id: &str) -> String {
    let mut logs = docker_logs(id).await.unwrap();
    let mut vec: Vec<String> = Vec::new();
    // let mut res: String = String::from("");

    while let Ok(Some(output)) = &logs.try_next().await {
        match output {
            LogOutput::StdOut { message } => {
                // res = String::from_utf8(message.to_vec()).unwrap();
                vec.push(String::from_utf8(message.to_vec()).unwrap());
            }
            LogOutput::StdErr { message } => {
                // res = String::from_utf8(message.to_vec()).unwrap();
                vec.push(String::from_utf8(message.to_vec()).unwrap());
            }
            _ => (),
        }
    }

    vec.iter_mut().for_each(|s| *s = format!("<p>{}</p>", s));

    docker_remove(id).await;

    let res = vec.into_iter().collect::<String>();

    res
    // let res = vec.into_iter().collect::<String>();
    // res
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![start, logs])
        .mount("/", FileServer::from("src/static"))
}
