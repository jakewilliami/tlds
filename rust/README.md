# Rust Library

For another project, I wanted the data from this Go library but in Rust.  This is a simple thing to implement using codegen.  It reads from [`data/tlds.json`](../data/tlds.json) and serialises it into a map.

```commandline
$ just  # runs checks against library and tests build
```

## Development

Remember to keep the library version specified in [`Cargo.toml`](./Cargo.toml) in line with the latest tag, at least when changing the data!
