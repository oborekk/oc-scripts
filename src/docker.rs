use bollard::container::{
    Config, CreateContainerOptions, LogOutput, LogsOptions, RemoveContainerOptions,
};
use bollard::Docker;
use futures_util::TryStreamExt;
use std::default::Default;
use std::io::{stdout, Write};

#[tokio::main]
pub async fn docker_image() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let docker = Docker::connect_with_socket_defaults().unwrap();

    let options = Some(CreateContainerOptions {
        name: "celsius-tests",
        platform: None,
    });

    // 519f1de54b92 > pinger
    // e052fcf8b981 > celsius
    let config = Config {
        image: Some("e052fcf8b981"),
        cmd: Some(vec!["10"]),
        ..Default::default()
    };

    let id = docker.create_container(options, config).await?.id;

    docker.start_container::<String>(&id, None).await?;

    let logopts = Some(LogsOptions::<String> {
        stdout: true,
        stderr: true,
        ..Default::default()
    });

    let mut logs = docker.logs(&id, logopts);

    while let Ok(Some(output)) = &logs.try_next().await {
        match output {
            LogOutput::StdOut { message } => stdout().write_all(message)?,
            LogOutput::StdErr { message } => stdout().write_all(message)?,
            _ => (),
        }
    }

    docker
        .remove_container(
            &id,
            Some(RemoveContainerOptions {
                force: true,
                ..Default::default()
            }),
        )
        .await?;

    Ok(())
}
