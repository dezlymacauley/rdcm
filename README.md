# 🦀 Rust Docker Container Manager 
_______________________________________________________________________________

A Rust-powered CLI tool that uses the Docker API

My goal is to create a CLI version of my `docker-lab` workflow,
which uses `mise` as a task runner.

- [docker-lab](https://github.com/dezlymacauley/dezly-labs/tree/main/docker-lab)
- [mise](https://github.com/jdx/mise)

_______________________________________________________________________________

### Implementation Checklist

Note: `sc` stands for subcommand

- main sc   sc           
- rdcm list containers 
- rdcm list containers --all (list all container)
- rdcm list images (list all images)
_______________________________________________________________________________
- main sc   sc    positional_argument           
- rdcm list start <container_id>  (start a specific container)
- rdcm list stop  <container_id> (stop a specific container)
- rdcm list pull <container_id> (pull a new Docker image)
_______________________________________________________________________________

### How to get Docker settings

Make sure that Docker is running and then run this command:

```bash
docker context inspect
```

Look for this line
```
"Host": "unix:///var/run/docker.sock"
```

This line tells the Docker CLI where to find the Docker Engine.

This is the specific part you use in `src/docker.rs`
`/var/run/docker.sock`
_______________________________________________________________________________

### Testing

To view a list of the subcommands that can be used with the 
list subcommand do this

```bash
cargo dev -- \
    list help
```

For the binary, this would be `rdcm list help`

_______________________________________________________________________________

To run `rdcm list containers`

```bash
cargo dev -- \
    list containers
```
_______________________________________________________________________________

To run `rdcm list containers --all`

```bash
cargo dev -- \
    list containers --all
```
_______________________________________________________________________________

To run `rdcm list images`

```bash
cargo dev -- \
    list images
```
_______________________________________________________________________________
