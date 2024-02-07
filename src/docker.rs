use bollard::container::{
    Config, CreateContainerOptions, LogOutput, LogsOptions, RemoveContainerOptions,
};
use bollard::errors::Error;
use bollard::Docker;
use futures_util::Stream;
use std::default::Default;

// Setup of docker container before starting it
pub async fn docker_setup(
    image_id: &str,
    parameters: &str,
) -> Result<String, Box<dyn std::error::Error + 'static>> {
    let docker = Docker::connect_with_unix_defaults().unwrap();

    let options = Some(CreateContainerOptions {
        name: "",
        platform: None,
    });

    let config = Config {
        image: Some(image_id),
        cmd: Some(vec![parameters]),
        ..Default::default()
    };

    let id = docker.create_container(options, config).await?.id;

    Ok(id)
}

// Logs stream creator
pub async fn docker_logs(
    id: &str,
) -> Result<impl Stream<Item = Result<LogOutput, Error>>, Box<dyn std::error::Error + 'static>> {
    let docker = Docker::connect_with_unix_defaults().unwrap();
    docker.start_container::<String>(&id, None).await?;

    let logopts = Some(LogsOptions::<String> {
        stdout: true,
        stderr: true,
        ..Default::default()
    });

    let logs = docker.logs(&id, logopts);

    Ok(logs)
}

// Container removal for cleanup
pub async fn docker_remove(id: String) -> Result<(), bollard::errors::Error> {
    let docker = Docker::connect_with_unix_defaults().unwrap();
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

// Uncomplete logging implementation
pub fn logger(result: Result<(), Error>) {
    match result {
        Ok(v) => dbg!(v),
        Err(e) => println!("{e}"),
    }
}
