#!/usr/bin/env rhai
print("Bygger projekt...");

let result = sh("cargo build --release");
if !result.ok {
    eprint(result.stderr);
    exit(1);
}

if path_exists("dist") {
    remove_file("dist");
}
make_dir("dist");
copy_file("target/release/rhai", "dist/rhai");
print("Klar i dist/");
