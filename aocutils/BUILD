load("@rules_rust//rust:defs.bzl", "rust_library", "rust_test")

package(default_visibility = ["//visibility:public"])

rust_library(
    name = "aocutils",
    srcs = ["src/lib.rs"],
)

rust_test(
    name = "aocutils_test",
    crate = ":aocutils",
)
