use std::{env::args, process::exit};

use anyhow::{Error, bail};
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

fn run(patterns: Vec<String>) -> Result<Vec<String>, Error> {
    if patterns.is_empty() {
        bail!("No patterns provided!");
    }
    let cache = new_cache!().unwrap();
    let mut lines = vec![];
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
            lines.push(serde_json::to_string(&dp).unwrap());
        }
    }
    Ok(lines)
}

fn main() {
    let patterns = args().skip(1).collect::<Vec<_>>();
    if let Err(err) = run(patterns) {
        println!("Error: {err}");
        exit(1);
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use rust_apt::config::Config;

    use crate::run;

    struct ResetConfig;
    impl Drop for ResetConfig {
        fn drop(&mut self) {
            Config::new().reset();
        }
    }

    #[test]
    fn empty_patterns() {
        assert_eq!(format!("{:?}", run(vec![])), "Err(No patterns provided!)");
    }

    #[test]
    fn demo_cache() {
        let reset = ResetConfig {};
        let config = Config::new();
        config.set("Dir", env::current_dir().unwrap().to_str().unwrap());
        config.set("Dir::Cache", "tests/");
        config.set("Dir::State", "tests/");
        assert_eq!(
            run(vec!["linux-image-6*".to_string()]).unwrap(),
            vec![
                "{\"name\":\"linux-image-6.8.12-amd64\",\"arch\":\"amd64\",\"versions\":[\"6.8.12-1\"],\"installed_version\":null}",
                "{\"name\":\"linux-image-6.1.0-21-amd64\",\"arch\":\"amd64\",\"versions\":[\"6.1.90-1\"],\"installed_version\":\"6.1.90-1\"}",
                "{\"name\":\"linux-image-6.10.9-amd64\",\"arch\":\"amd64\",\"versions\":[\"6.10.9-1\"],\"installed_version\":null}",
                "{\"name\":\"linux-image-6.1.0-25-amd64\",\"arch\":\"amd64\",\"versions\":[\"6.1.106-3\"],\"installed_version\":\"6.1.106-3\"}"
            ],
        );
        drop(reset);
    }
}
