# Key Value (KV) store CLI tool written in Rust.

To build, run:

```bash
cargo build --release
```

Copy the _kvstore_ file to your desired location and run it like the following:

```bash
./kvstore foo bar #./kvstore.exe on Windows
```

This will generate _kv.db_ which contains all key value pairs.
