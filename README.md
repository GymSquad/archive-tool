# Archive Tool

A command line tool for validating and archiving records stored in the database (written in Rust)

## Usage

### Using Docker

Pull the Docker Image from [Docker Hub](https://hub.docker.com/repository/docker/alan910127/archive-tool/general)

```bash
docker pull alan910127/archive-tool
```

Run the Application

```bash
docker run --rm -it \
    --add-host host.docker.internal:host-gateway \
    -e DATABASE_URL="postgresql://app:app@host.docker.internal:5432/db" \
    -v <pywb-collections-path>:/data \
    alan910127/archive-tool
```

> If the database is located at `localhost`, you should use `host.docker.internal` with `--add-host host.docker.internal:host-gateway`.

### Build from Source

Clone the Repository

```bash
git clone --branch rs https://github.com/GymSquad/archive-tool.git
```

Change Your Current Directory

```bash
cd archive-tool
```

Build the Binary

```bash
cargo build --release && cp target/release/archive-tool .
```

Run the Program

```bash
./archive-tool [pywb collections path]
```
