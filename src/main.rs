use std::{
    collections::HashSet,
    fs,
    process::{Command, Stdio},
};

use clap::Parser;
use config::Args;

mod config;

fn main() {
    let args = Args::parse();

    let files = changed_files(&args.since);
    let report = logana_files();
    let original = report.len();

    let mut out = String::new();
    let mut count = 0;

    report
        .iter()
        .filter(|message| {
            files
                .iter()
                .find(|file| message.contains(&**file))
                .is_some()
        })
        .for_each(|m| {
            count += 1;
            out.push_str(m);
            out.push('\n');
        });

    fs::write(".logana-report", out).expect("Unable to save .logana-report");
    println!("There where {} messages now there are {}", original, count);
}

fn logana_files() -> Vec<String> {
    let file = fs::read_to_string(".logana-report").expect("Unable to read file");
    let files: Vec<String> = file
        .lines()
        .map(|f| f.replace('\\', "/"))
        .map(|a| a.to_string())
        .collect();

    files
}

fn changed_files(since: &str) -> HashSet<String> {
    let git_output = Command::new("git")
        .args(["diff", "--name-only", since])
        .stdout(Stdio::piped())
        .output()
        .expect("To get output")
        .stdout;
    let git = String::from_utf8(git_output).expect("Found invalid UTF-8");
    let files: HashSet<String> = git.lines().map(|a| a.to_string()).collect();

    files
}
