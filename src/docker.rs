use bollard::errors::Error;
use bollard::models::{ContainerSummary, ImageSummary};
use bollard::query_parameters::{ListContainersOptionsBuilder, ListImagesOptionsBuilder};
use bollard::Docker;

pub struct DockerClient {
    docker: Docker,
}

impl DockerClient {
    pub fn new() -> Self {
        // Option A: Use local defaults (automatically handles Unix socket or Windows pipe)
        let docker = Docker::connect_with_local_defaults()
            .expect("Failed to connect to Docker socket");

        // Option B: Explicit Unix socket (if you prefer your previous explicit setup)
        /*
        let docker = Docker::connect_with_unix(
            "/var/run/docker.sock",
            120,
            bollard::API_DEFAULT_VERSION,
        )
        .expect("Failed to connect to Docker");
        */

        Self { docker }
    }

    pub async fn list_containers(&self, all: bool) -> Result<Vec<ContainerSummary>, Error> {
        // Build query parameters using ListContainersOptionsBuilder
        let options = ListContainersOptionsBuilder::default()
            .all(all)
            .build();

        // Pass options to list_containers
        let containers = self.docker.list_containers(Some(options)).await?;
        Ok(containers)
    }

    pub async fn list_images(&self) -> Result<Vec<ImageSummary>, Error> {
        // Build image listing options using the builder API
        let options = ListImagesOptionsBuilder::default()
            .all(true)
            .build();

        let images = self.docker.list_images(Some(options)).await?;
        Ok(images)
    }

}
