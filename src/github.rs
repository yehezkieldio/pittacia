use regex::Regex;

///
/// Extract username and repository name from a GitHub repository URL, either HTTPS or SSH.
///
pub fn extract_from_link(link: &str) -> (String, String) {
    let re = Regex::new(r#"(?:https?|git)://github\.com/(?P<username>[^/]+)/(?P<repo>[^/]+)(?:\.git)?|git@github\.com:(?P<username_ssh>[^/]+)/(?P<repo_ssh>[^/]+)(?:\.git)?"#).unwrap();

    if let Some(captures) = re.captures(link) {
        if let (Some(username), Some(repo)) = (captures.name("username"), captures.name("repo")) {
            return (
                username.as_str().to_string(),
                clean_repo_name(repo.as_str()),
            );
        } else if let (Some(username_ssh), Some(repo_ssh)) =
            (captures.name("username_ssh"), captures.name("repo_ssh"))
        {
            return (
                username_ssh.as_str().to_string(),
                clean_repo_name(repo_ssh.as_str()),
            );
        }
    }

    ("".to_string(), "".to_string())
}

fn clean_repo_name(repo: &str) -> String {
    if repo.ends_with(".git") {
        repo[..repo.len() - 4].to_string()
    } else {
        repo.to_string()
    }
}
