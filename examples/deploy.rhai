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
sh_ok("scp target/release/rhai user@server:/opt/rhai");
print("Deploy færdig!");
