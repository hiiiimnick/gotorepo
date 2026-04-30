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

    let binding = String::from_utf8_lossy(&output.stdout).to_string();

    if binding.starts_with("git@") {
        let url = from_ssh(binding);
        let result = open::that(url);
        result.report();
    } else {
        let result = open::that(from_http(binding));
        result.report();
    }
}

pub fn from_ssh(binding: String) -> String {
    let temp = binding.replacen(":", "/", 1);
    let url = temp.strip_prefix("git@").expect("fatal error");
    "https://".to_owned() + url.trim()
}

pub fn from_http(binding: String) -> String {
    let url = if binding.starts_with("http") {
        binding.trim().to_owned()
    } else {
        let mut temp = String::from("https://");
        temp.push_str(&binding);
        temp
    };
    url
}

#[cfg(test)]
mod tests {
    use crate::from_http;
    use crate::from_ssh;

    #[test]
    fn test_ssh() {
        let ssh = String::from("git@github.com:testuser/testrepo.git");
        let expected_result = String::from("https://github.com/testuser/testrepo.git");
        assert_eq!(expected_result, from_ssh(ssh));
    }

    #[test]
    fn test_complete_http() {
        let http = String::from("https://github.com/testuser/testrepo.git");
        let expected_result = String::from("https://github.com/testuser/testrepo.git");
        assert_eq!(expected_result, from_http(http));
    }

    #[test]
    fn test_missing_http_prefix() {
        let http = String::from("github.com/testuser/testrepo.git");
        let expected_result = String::from("https://github.com/testuser/testrepo.git");
        assert_eq!(expected_result, from_http(http));
    }
}
