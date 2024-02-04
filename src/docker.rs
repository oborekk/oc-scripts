use bollard::container::{
    Config, CreateContainerOptions, LogOutput, LogsOptions, RemoveContainerOptions,
};
use bollard::errors::Error;
use bollard::Docker;
use futures_util::{Stream, TryStreamExt};
use std::default::Default;
use std::io::{stdout, Write};

#[tokio::main]
pub async fn docker_image(
    container: &str,
) -> Result<impl Stream<Item = Result<LogOutput, Error>>, Box<dyn std::error::Error + 'static>> {
    // pub async fn docker_image(container: &str) ->impl Stream<Item = Result<LogOutput, Error>> {
    let docker = Docker::connect_with_socket_defaults().unwrap();

    let options = Some(CreateContainerOptions {
        name: "celsius-tests",
        platform: None,
    });

    // 519f1de54b92 > pinger
    // e052fcf8b981 > celsius
    let config = Config {
        image: Some(container),
        cmd: Some(vec!["10"]),
        ..Default::default()
    };

    let id = docker.create_container(options, config).await?.id;
    println!("CONTAINER ID {id}");

    docker.start_container::<String>(&id, None).await?;

    let logopts = Some(LogsOptions::<String> {
        stdout: true,
        stderr: true,
        ..Default::default()
    });

    let mut logs = docker.logs(&id, logopts);

    // while let Ok(Some(output)) = &logs.try_next().await {
    //     match output {
    //         LogOutput::StdOut { message } => stdout().write_all(message)?,
    //         LogOutput::StdErr { message } => stdout().write_all(message)?,
    //         _ => (),
    //     }
    // }

    // docker
    //     .remove_container(
    //         &id,
    //         Some(RemoveContainerOptions {
    //             force: true,
    //             ..Default::default()
    //         }),
    //     )
    //     .await?;

    Ok(logs)
}
