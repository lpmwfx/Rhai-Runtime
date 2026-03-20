# rhai — A General-Purpose Rhai Scripting Runtime

> Rhai som et nyt Python. Kør `.rhai` scripts direkte fra terminalen med adgang til filsystem, shell, miljøvariabler og HTTP.

---

## Motivation

Rhai er designet som et embedded scripting library til Rust — ikke som en standalone runtime. Det betyder at der ikke findes noget á la `python script.py` for Rhai. Dette projekt bygger netop det: en CLI binary der giver Rhai de samme muligheder som Python har som generelt scripting- og toolingsprog.

```bash
# I stedet for:
python build.py

# Kan du skrive:
rhai build.rhai
```

---

## Projektstruktur

```
rhai-runtime/
├── Cargo.toml
├── src/
│   └── main.rs          # CLI entry point + API registrering
├── std/                 # Standard library scripts (importerbare)
│   ├── path.rhai
│   └── log.rhai
├── examples/
│   ├── build.rhai
│   ├── deploy.rhai
│   └── http_fetch.rhai
└── README.md
```

---

## CLI Interface

```bash
rhai <script.rhai> [args...]
rhai --help
```

### Eksempler

```bash
rhai build.rhai
rhai deploy.rhai --env prod
rhai fetch.rhai https://api.github.com/repos/rust-lang/rust
```

### Shebang support

```rhai
#!/usr/bin/env rhai
print("hello world");
```

---

## Built-in API

### Filsystem

| Funktion | Beskrivelse |
|---|---|
| `read_file(path)` | Returnerer filindhold som string |
| `write_file(path, content)` | Skriver/overskriver fil |
| `append_file(path, content)` | Tilføjer til fil |
| `remove_file(path)` | Sletter fil eller mappe (rekursivt) |
| `copy_file(src, dst)` | Kopierer fil |
| `move_file(src, dst)` | Flytter/omdøber fil |
| `make_dir(path)` | Opretter mappe (inkl. parents) |
| `read_dir(path)` | Returnerer array af stier i mappen |
| `path_exists(path)` | `true`/`false` |
| `is_file(path)` | `true`/`false` |
| `is_dir(path)` | `true`/`false` |
| `file_size(path)` | Filstørrelse i bytes |

### Shell / Process

| Funktion | Beskrivelse |
|---|---|
| `sh(cmd)` | Kører shell-kommando, returnerer `#{ stdout, stderr, code, ok }` |
| `sh_ok(cmd)` | Kører kommando, printer output, afslutter ved fejl |
| `exit(code)` | Afslutter processen med exit code |
| `args()` | Array af script-argumenter |

```rhai
let result = sh("cargo build --release");
if !result.ok {
    eprint("Build fejlede: " + result.stderr);
    exit(1);
}
```

### Miljøvariabler

| Funktion | Beskrivelse |
|---|---|
| `env(key)` | Henter miljøvariabel (fejler hvis ikke sat) |
| `env_or(key, default)` | Henter miljøvariabel med fallback |
| `set_env(key, val)` | Sætter miljøvariabel |
| `env_map()` | Alle miljøvariabler som map |

### HTTP

| Funktion | Beskrivelse |
|---|---|
| `http_get(url)` | GET request, returnerer body som string |
| `http_get_json(url)` | GET request, returnerer parset JSON som Rhai map |
| `http_post(url, body)` | POST request med JSON body |

```rhai
let data = http_get_json("https://api.github.com/repos/rhaiscript/rhai");
print("Stars: " + data.stargazers_count);
```

---

## Eksempel Scripts

### build.rhai

```rhai
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
copy_file("target/release/myapp", "dist/myapp");
print("Klar i dist/");
```

### deploy.rhai

```rhai
#!/usr/bin/env rhai
let script_args = args();
let env_name = if script_args.len() > 0 { script_args[0] } else { "staging" };

let token = env_or("DEPLOY_TOKEN", "");
if token == "" {
    eprint("Mangler DEPLOY_TOKEN");
    exit(1);
}

print("Deployer til " + env_name + "...");
sh_ok("cargo build --release");
sh_ok("scp target/release/myapp user@server:/opt/myapp");
print("Deploy færdig!");
```

### fetch.rhai

```rhai
#!/usr/bin/env rhai
let url = args()[0];
let data = http_get_json(url);
write_file("output.json", data.to_string());
print("Gemt til output.json");
```

---

## Dependencies (Cargo.toml)

```toml
[dependencies]
rhai    = { version = "1", features = ["serde"] }
ureq    = { version = "2", features = ["json"] }
serde_json = "1"
```

---

## Roadmap

### v0.1 — MVP
- [x] `rhai script.rhai [args...]`
- [x] Filsystem API
- [x] Shell API (`sh`, `sh_ok`)
- [x] Miljøvariabler
- [x] HTTP (`http_get`, `http_post`, `http_get_json`)
- [x] Shebang support
- [x] `--help`

### v0.2 — Brugbarhed
- [ ] `rhai watch script.rhai` — re-kør ved filændring
- [ ] Farvet fejloutput med linje/kolonne info
- [ ] Moduler — `import "std/path"` fra en global std-lib
- [ ] `glob(pattern)` — find filer med wildcards

### v0.3 — Pakker
- [ ] `rhai install` — hent og cache Rhai-moduler
- [ ] Lokalt `rhai_modules/` i projekt-root
- [ ] Simpel `rhai.toml` til scriptkonfiguration

---

## Installation (når bygget)

```bash
cargo install --path .

# Eller via cargo fra crates.io (fremtidigt):
cargo install rhai-run
```

---

## Filosofi

- **Rhai er syntaksen** — ingen ny syntaks opfindes
- **Rust er motoren** — APIs implementeres i Rust og eksponeres til scripts
- **Python er forbilledet** — det skal være ligeså nemt at skrive `rhai script.rhai` som `python script.py`
- **Ingen magic** — scripts er plain `.rhai` filer, ingen config påkrævet


---

<!-- LARS:START -->
<a href="https://lpmathiasen.com">
  <img src="https://carousel.lpmathiasen.com/carousel.svg?slot=5" alt="Lars P. Mathiasen"/>
</a>
<!-- LARS:END -->
