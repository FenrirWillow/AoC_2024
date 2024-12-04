load("@crates//:defs.bzl", "aliases", "all_crate_deps")
load("@rules_rust//rust:defs.bzl", "rust_library", "rust_test")

package(default_visibility = ["//visibility:public"])

rust_library(
    name = "aocutils",
    srcs = ["src/lib.rs"],
    aliases = aliases(),
    proc_macro_deps = all_crate_deps(
        proc_macro = True,
    ),
    deps = all_crate_deps(
        normal = True,
    ),
)

rust_test(
    name = "aocutils_test",
    aliases = aliases(
        normal_dev = True,
        proc_macro_dev = True,
    ),
    crate = ":aocutils",
    proc_macro_deps = all_crate_deps(
        proc_macro_dev = True,
    ),
    deps = all_crate_deps(
        normal_dev = True,
    ),
)
