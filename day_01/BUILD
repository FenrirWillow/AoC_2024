load("@crates//:defs.bzl", "all_crate_deps")
load("@rules_rust//rust:defs.bzl", "rust_binary")

rust_binary(
    name = "day_01",
    srcs = ["src/main.rs"],
    deps = ["//aocutils"] + all_crate_deps(normal = True),
)
