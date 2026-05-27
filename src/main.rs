use std::env::args;

use fast_glob::glob_match;
use rust_apt::new_cache;
use serde::Serialize;

#[derive(Serialize)]
struct DumpPackage<'a> {
    name: &'a str,
    arch: &'a str,
    versions: Vec<String>,
    installed_version: Option<String>,
}

fn main() {
    let patterns = args().skip(1).collect::<Vec<_>>();
    if patterns.is_empty() {
        println!("No patterns provided!");
        return;
    }
    let cache = new_cache!().unwrap();
    for pattern in patterns {
        for package in cache
            .iter()
            .filter(|p| p.versions().any(|_| true) && glob_match(&pattern, p.name()))
        {
            let dp = DumpPackage {
                name: package.name(),
                versions: package
                    .versions()
                    .map(|v| v.version().to_string())
                    .collect(),
                arch: package.arch(),
                installed_version: package.installed().map(|v| v.version().to_string()),
            };
            println!("{}", serde_json::to_string(&dp).unwrap());
        }
    }
}
