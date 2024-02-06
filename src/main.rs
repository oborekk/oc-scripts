pub mod docker;

use crate::docker::docker_setup;
use bollard::container::LogOutput;
use docker::{docker_logs, docker_remove, logger};
use futures_util::TryStreamExt;
use rocket::form::Form;
use rocket::fs::FileServer;
use rocket::response::stream::{Event, EventStream};
use rocket::{launch, routes};
#[macro_use]
extern crate rocket;

#[derive(FromForm)]
struct Params<'r> {
    params: &'r str,
}

#[post("/start/<option>", data = "<params>")]
async fn start(option: &str, params: Form<Params<'_>>) -> String {
    let id = docker_setup(option, params.params).await;
    match id {
        Ok(v) => {
            format!("
            <div id=\"sse{v}\">
                <div hx-target=\"#terminal-{option}\" hx-ext=\"sse\" sse-swap=\"message\" sse-connect=\"/logs/{v}\"></div>
            </div>")
        }
        Err(e) => {
            format!("<div>Error! {e}</div>")
        }
    }
}

#[get("/logs/<id>")]
async fn logs(id: String) -> EventStream![] {
    let mut logs = docker_logs(&id).await.unwrap();
    // let mut lifetime = 0;
    // let mut interval = time::interval(Duration::from_secs(1));
    EventStream! {
        let mut vec: Vec<String>  = Vec::new();
        while let Ok(output) = &logs.try_next().await {
            match output {
                Some(v) => match v {
                    LogOutput::StdOut { message } => {
                        vec.push(format!("<pre data-prefix=\">\"><code>{}</code></pre>", String::from_utf8(message.to_vec()).unwrap()));
                        let res = vec.clone().into_iter().collect::<String>();
                        yield Event::data(res)
                    }
                    LogOutput::StdErr { message } => {
                        vec.push(format!("<pre data-prefix=\">\"><code>{}</code></pre>", String::from_utf8(message.to_vec()).unwrap()));
                        let res = vec.clone().into_iter().collect::<String>();
                        yield Event::data(res)
                    }
                    _ => (),
                },
                None => {
                    let removal = docker_remove(id.clone()).await;
                    logger(removal);
                    break
                }
            }
        }
        vec.push(format!("<div hx-trigger=\"load\" hx-target=\"#sse{}\" hx-get=\"/stop\"></div>", id));
        let res = vec.clone().into_iter().collect::<String>();
        yield Event::data(res)
    }
}

#[get("/stop")]
fn stop() -> String {
    String::from("<div></div>")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![start, logs, stop])
        .mount("/", FileServer::from("src/static"))
}
