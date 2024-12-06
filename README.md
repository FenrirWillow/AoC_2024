# Advent of Code 2024

---

## Overview

This repository contains my solutions for "Advent of Code 2024".

## Getting started

### Required tools:

- [Bazelisk](https://bazel.build/install/bazelisk) - used to manage Bazel, which in turn is used to manage all compilers, dependencies, libraries and binaries, and their tasks.

- [Rust](https://www.rust-lang.org/) - The Rust programming language (the toolchain itself is managed by Bazel, but you will likely need a local copy in order to use with your IDE/editor)

### Basics

- Run a single day: `bazel run --config=release //day_<day number> \-- <args>`

- Run tests for a single day: `bazel test //day_<day number>:all`

- If you update any of the dependencies, you will need to regenerate the crate index. To do that, run the following command: `CARGO_BAZEL_REPIN=1 bazel sync --only=crates`

## Bootstrapping a new day

To add a new day to the repository, follow this list of instructions:

1. Run `cargo new day_<day number> --bin`. This should produce a new folder for the source code of that day, as well as add it to the root `Cargo.toml`.

2. Add an entry for the new folder to the `MODULE.bazel` file in the `crates.from_cargo` manifests. This will allow Bazel to graph and build the dependencies for the binary, as well as the binary itself.

3. Add any basic dependencies to the day's `Cargo.toml`. Most days use the shared `aocutils` library, as well as dependencies like [`clap`](https://docs.rs/clap/latest/clap/) and [`itertools`](https://docs.rs/itertools/latest/itertools/).

```toml
[dependencies]
aocutils = { path = "../aocutils" }
clap = { version = "4.5.22", features = ["derive", "cargo"] }
itertools = { version = "0.13.0" }
```

4. Create a `BUILD` file for the new day. You can use the other day's build files as a template, but it should generally look something like this:

```starklang
load("@crates//:defs.bzl", "all_crate_deps")
load("@rules_rust//rust:defs.bzl", "rust_binary")

rust_binary(
    name = "day_<day number>",
    srcs = ["src/main.rs"],
    data = glob(["inputs/*.txt"]),
    deps = [
        "//aocutils",
    ] + all_crate_deps(normal = True),
)
```

5. Bootstrap the basic day solving program - most days start off the same, so starting off with the common main is a good idea.

```rust
use clap::Parser;
use std::{env, path};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "example.txt")]
    input: String,
}

fn main() {
    let args = Args::parse();
    let crate_name = clap::crate_name!();
    let cli_inputpath = path::Path::new(&args.input);

    println!("Hello to solver for {crate_name}");

    // TODO: (Stefan) Solve today's problem! ⭐⭐
}

```

6. Create a subfolder in the day called `inputs` with `example.txt` as a file. This file can then be filled with the example input given by Advent of Code.

7. Resync all dependencies - only needs to be done once, since we are adding a new target.

```bash
CARGO_BAZEL_REPIN=1 bazel sync --only=crates
```

8. Build and run the new day

```bash
bazel run --config=debug //day_<day number> -- --input="example.txt"
```
