use std::fs;

use regex::Regex;

#[derive(Debug)]
#[allow(dead_code)]
pub struct GitHub {
    pub username: String,
    pub repo: String,
}

///
/// Extract username and repository name from a GitHub repository URL, either HTTPS or SSH.
///
pub fn extract_from_link(link: &str) -> Result<GitHub, ()> {
    let re = Regex::new(r#"(?:https?|git)://github\.com/(?P<username>[^/]+)/(?P<repo>[^/]+)(?:\.git)?|git@github\.com:(?P<username_ssh>[^/]+)/(?P<repo_ssh>[^/]+)(?:\.git)?"#).unwrap();

    if let Some(captures) = re.captures(link) {
        if let (Some(username), Some(repo)) = (captures.name("username"), captures.name("repo")) {
            return Ok(GitHub {
                username: username.as_str().to_string(),
                repo: clean_repo_name(repo.as_str()),
            });
        } else if let (Some(username_ssh), Some(repo_ssh)) =
            (captures.name("username_ssh"), captures.name("repo_ssh"))
        {
            return Ok(GitHub {
                username: username_ssh.as_str().to_string(),
                repo: clean_repo_name(repo_ssh.as_str()),
            });
        }
    }

    Err(())
}

///
/// Extract username and repository name from the current directory's Git configuration.
///
pub fn extract_from_local() -> Result<GitHub, ()> {
    let current_dir = std::env::current_dir().unwrap();
    let current_dir = current_dir.to_str().unwrap();

    let config_file = format!("{}/.git/config", current_dir);
    let config_content = fs::read_to_string(&config_file).unwrap();
    let re = Regex::new(r#"\[remote "(origin|upstream)"\]\n\s*url = (?P<url>.+)\n"#).unwrap();

    if let Some(captures) = re.captures(&config_content) {
        if let Some(url) = captures.name("url") {
            return extract_from_link(url.as_str());
        }
        return Err(());
    }

    Err(())
}

fn clean_repo_name(repo: &str) -> String {
    if repo.ends_with(".git") {
        repo[..repo.len() - 4].to_string()
    } else {
        repo.to_string()
    }
}
