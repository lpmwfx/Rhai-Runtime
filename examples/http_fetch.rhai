#!/usr/bin/env rhai
let url = args()[0];
let data = http_get_json(url);
write_file("output.json", data.to_string());
print("Gemt til output.json");
