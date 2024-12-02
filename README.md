# Advent of Code 2024

---

## Overview

This repository contains my solutions for "Advent of Code 2024".

## Getting started

### Required tools:

- [Bazelisk](https://bazel.build/install/bazelisk) - used to manage Bazel, which in turn is used to manage all compilers, dependencies, libraries and binaries, and their tasks.

- [Rust](https://www.rust-lang.org/) - The Rust programming language (the toolchain itself is managed by Bazel, but you will likely need a local copy in order to use with your IDE/editor)

### Basics

- Run a single day: `bazel run //day_<day number>`

- Run tests for a single day: `bazel test //day_<day number>:all`
