# Archive Tool

A command line tool for validating and archiving records stored in the database (written in Rust)

## Usage

```
Usage: archive-tool [OPTIONS] --accept-formats <ACCEPT_FORMATS> --output <OUTPUT>

Options:
  -a, --accept-formats <ACCEPT_FORMATS>
          path to the accepted file format configuration file
  -o, --output <OUTPUT>
          path to the output directory
  -d, --database-url <DATABASE_URL>
          database URL (priority over DATABASE_URL env var)
  -n, --num-url <NUM_URL>
          the number of URLs to archive, useful for testing (default: no limit)
  -t, --tasks <TASKS>
          the maxmimum number of concurrent tasks (default: 4)
  -h, --help
          Print help
```

## Run with Docker

```bash
docker run -it --rm \
  --volume "$(pwd):/app" \
  --user "$(id -u):$(id -g)" \
  alan910127/archive-tool:latest \
  --accept-formats /app/accept.txt \
  --output /app/archive \
  --database-url "postgresql://app:app@localhost:5432/db"
```
