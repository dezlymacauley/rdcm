# 🦀 Rust Docker Container Manager 
_______________________________________________________________________________

A Rust-powered CLI tool that uses the Docker API

My goal is to create a CLI version of my `docker-lab` workflow,
which uses `mise` as a task runner.

- [docker-lab](https://github.com/dezlymacauley/dezly-labs/tree/main/docker-lab)
- [mise](https://github.com/jdx/mise)

_______________________________________________________________________________

### Implementation Checklist

- main sc   sc           
- rdcm list containers (list all container)
- rdcm list images (list all images)
_______________________________________________________________________________
- main sc   sc    positional_argument           
- rdcm list start <container_id>  (start a specific container)
- rdcm list stop  <container_id> (stop a specific container)
- rdcm list pull <container_id> (pull a new Docker image)
_______________________________________________________________________________
