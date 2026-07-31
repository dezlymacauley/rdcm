use bollard::Docker;

pub struct DockerClient {
    docker: Docker,
}

impl DockerClient {
    pub fn new() -> Self {
        let docker = Docker::connect_with_unix(
            // path, timeout, client_version
            "/var/run/docker.sock",
            120,
            bollard::API_DEFAULT_VERSION,
        ).expect("Failed to connect to Docker");

        Self {
            docker: docker
        }
    }
}
