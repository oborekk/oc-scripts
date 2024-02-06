use bollard::container::{
    Config, CreateContainerOptions, LogOutput, LogsOptions, RemoveContainerOptions,
};
use bollard::errors::Error;
use bollard::Docker;
use futures_util::Stream;
use std::default::Default;
use tokio::io::{stdout, AsyncWriteExt};

pub async fn docker_setup(image_id: &str) -> Result<String, Box<dyn std::error::Error + 'static>> {
    let docker = Docker::connect_with_socket_defaults().unwrap();

    let options = Some(CreateContainerOptions {
        name: "",
        platform: None,
    });

    // 519f1de54b92 > pinger
    // e052fcf8b981 > celsius
    let config = Config {
        image: Some(image_id),
        cmd: Some(vec!["10"]),
        ..Default::default()
    };

    let id = docker.create_container(options, config).await?.id;
    // println!("CONTAINER ID {id}");
    stdout()
        .write_all(b"Container created with ID {id}")
        .await?;

    Ok(id)
}

pub async fn docker_logs(
    id: &str,
) -> Result<impl Stream<Item = Result<LogOutput, Error>>, Box<dyn std::error::Error + 'static>> {
    let docker = Docker::connect_with_socket_defaults().unwrap();
    docker.start_container::<String>(&id, None).await?;

    let logopts = Some(LogsOptions::<String> {
        stdout: true,
        stderr: true,
        ..Default::default()
    });

    let logs = docker.logs(&id, logopts);

    Ok(logs)
}

pub async fn docker_remove(id: &str) -> Result<(), bollard::errors::Error> {
    let docker = Docker::connect_with_socket_defaults().unwrap();
    docker
        .remove_container(
            &id,
            Some(RemoveContainerOptions {
                force: true,
                ..Default::default()
            }),
        )
        .await
}
