use std::process::{Command, Termination};

fn main() {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg("git config --get remote.origin.url")
            .output()
            .expect("no remote url found, not in a git repo?")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("git config --get remote.origin.url")
            .output()
            .expect("no remote url found, not in a git repo?")
    };

    let binding = String::from_utf8_lossy(&output.stdout);

    if binding.starts_with("git@") {
        let temp = binding.replacen(":", "/", 1);
        let url = temp.strip_prefix("git@").expect("fatal error");
        let result = open::that("http://".to_owned() + url.trim());
        result.report();
    } else {
        let result = open::that(binding.trim());
        result.report();
    }
}
