fn join(base, part) {
    if base.ends_with("/") || base.ends_with("\\") {
        base + part
    } else {
        base + "/" + part
    }
}

fn basename(path) {
    let parts = path.split("/");
    parts[parts.len() - 1]
}

fn dirname(path) {
    let idx = path.last_index_of("/");
    if idx < 0 { "." } else { path.sub_string(0, idx) }
}
