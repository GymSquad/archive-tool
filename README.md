# Archive Tool

A command line tool for validating and archiving records stored in the database (written in Rust)

## Usage

```
Usage: archive-tool [OPTIONS] --config <CONFIG> --output <OUTPUT>

Options:
  -c, --config <CONFIG>              path to the file format blacklist configuration file
  -o, --output <OUTPUT>              path to the output directory
  -d, --database-url <DATABASE_URL>  database URL
  -n, --num-url <NUM_URL>            the number of URLs to archive (useful for testing) (default: no limit)
  -h, --help                         Print help
```
