# rhai-runtime

A standalone CLI binary that turns [Rhai](https://rhai.rs) into a general-purpose scripting runtime — the missing `rhai script.rhai` equivalent of `python script.py`.

Rust is the engine. The binary exposes filesystem, shell, environment, and HTTP APIs to Rhai scripts. No new syntax is invented — Rhai is the language, Rust is the host.

```bash
rhai build.rhai
rhai deploy.rhai prod
rhai fetch.rhai https://api.github.com/repos/rhaiscript/rhai
```

## Installation

```bash
cargo install --path .
```

Requires a stable Rust toolchain.

## Usage

```
rhai <script.rhai> [args...]
rhai --help
```

### Shebang support

```rhai
#!/usr/bin/env rhai
print("hello world");
```

Mark the script executable and run it directly on Unix.

## Built-in API

### Filesystem

| Function | Description |
|---|---|
| `read_file(path)` | Returns file contents as a string |
| `write_file(path, content)` | Writes (or overwrites) a file |
| `append_file(path, content)` | Appends to a file |
| `remove_file(path)` | Deletes a file or directory (recursive) |
| `copy_file(src, dst)` | Copies a file |
| `move_file(src, dst)` | Moves or renames a file |
| `make_dir(path)` | Creates a directory (including parents) |
| `read_dir(path)` | Returns an array of paths in the directory |
| `path_exists(path)` | `true` / `false` |
| `is_file(path)` | `true` / `false` |
| `is_dir(path)` | `true` / `false` |
| `file_size(path)` | File size in bytes |

### Shell / Process

| Function | Description |
|---|---|
| `sh(cmd)` | Runs a shell command, returns `#{ stdout, stderr, code, ok }` |
| `sh_ok(cmd)` | Runs a command, prints output, exits on failure |
| `exit(code)` | Exits the process with the given exit code |
| `args()` | Array of arguments passed to the script |

```rhai
let result = sh("cargo build --release");
if !result.ok {
    print("Build failed: " + result.stderr);
    exit(1);
}
```

### Environment

| Function | Description |
|---|---|
| `env(key)` | Gets an environment variable (throws if not set) |
| `env_or(key, default)` | Gets an environment variable with a fallback |
| `set_env(key, val)` | Sets an environment variable |
| `env_map()` | All environment variables as a map |

### HTTP

| Function | Description |
|---|---|
| `http_get(url)` | GET request, returns body as string |
| `http_get_json(url)` | GET request, returns parsed JSON as a Rhai map |
| `http_post(url, body)` | POST request with a JSON body |

```rhai
let data = http_get_json("https://api.github.com/repos/rhaiscript/rhai");
print("Stars: " + data.stargazers_count);
```

## Example Scripts

### build.rhai

```rhai
#!/usr/bin/env rhai
print("Building project...");

let result = sh("cargo build --release");
if !result.ok {
    print(result.stderr);
    exit(1);
}

if path_exists("dist") {
    remove_file("dist");
}
make_dir("dist");
copy_file("target/release/myapp", "dist/myapp");
print("Done. Output in dist/");
```

### deploy.rhai

```rhai
#!/usr/bin/env rhai
let script_args = args();
let target_env = if script_args.len() > 0 { script_args[0] } else { "staging" };

let token = env_or("DEPLOY_TOKEN", "");
if token == "" {
    print("Error: DEPLOY_TOKEN not set");
    exit(1);
}

print("Deploying to " + target_env + "...");
sh_ok("cargo build --release");
sh_ok("scp target/release/myapp user@server:/opt/myapp");
print("Deploy complete.");
```

### fetch.rhai

```rhai
#!/usr/bin/env rhai
let url = args()[0];
let data = http_get_json(url);
write_file("output.json", data.to_string());
print("Saved to output.json");
```

## Project Structure

```
rhai-runtime/
├── Cargo.toml
├── src/
│   ├── main.rs          # CLI entry point and API wiring
│   ├── api/
│   │   ├── fs.rs        # Filesystem API
│   │   ├── shell.rs     # Shell / process API
│   │   ├── env.rs       # Environment API
│   │   └── http.rs      # HTTP API
│   └── state/
│       ├── sizes.rs     # Buffer and limit constants
│       └── paths.rs     # Path constants (shebang prefix, etc.)
├── std/                 # Importable Rhai standard library modules
├── examples/            # Example scripts
└── doc/                 # Design documentation
```

## Stack

- **Scripting:** [rhai](https://crates.io/crates/rhai) with `serde` feature
- **HTTP:** [ureq](https://crates.io/crates/ureq) v2
- **Serialisation:** [serde_json](https://crates.io/crates/serde_json)
- **Error handling:** [thiserror](https://crates.io/crates/thiserror) (core), [anyhow](https://crates.io/crates/anyhow) (CLI boundary)
- **Logging:** [tracing](https://crates.io/crates/tracing) + [tracing-subscriber](https://crates.io/crates/tracing-subscriber)

## Roadmap

### v0.1 — MVP
- [x] `rhai script.rhai [args...]`
- [x] Filesystem API
- [x] Shell API (`sh`, `sh_ok`)
- [x] Environment variables
- [x] HTTP (`http_get`, `http_post`, `http_get_json`)
- [x] Shebang support
- [x] `--help`

### v0.2 — Usability
- [ ] `rhai watch script.rhai` — re-run on file change
- [ ] Coloured error output with line/column info
- [ ] Modules — `import "std/path"` from a global stdlib
- [ ] `glob(pattern)` — find files with wildcards

### v0.3 — Packages
- [ ] `rhai install` — fetch and cache Rhai modules
- [ ] Local `rhai_modules/` in project root
- [ ] Simple `rhai.toml` for script configuration

## Philosophy

- **Rhai is the syntax** — no new language constructs are invented
- **Rust is the engine** — APIs are implemented in Rust and exposed to scripts
- **Python is the model** — `rhai script.rhai` should feel as natural as `python script.py`
- **No magic** — scripts are plain `.rhai` files, no config required

## License

MIT


---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=5" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
