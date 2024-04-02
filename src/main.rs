use argh::FromArgs;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use regex::Regex;

#[derive(FromArgs)]
/// Experimental CLI for managing GitHub issues label.
struct Pittacia {}

fn extract_info_from_gh_link(link: &str) -> (String, String) {
    let re = Regex::new(r"https://github\.com/(?P<username>[^/]+)/(?P<repo>[^/]+)").unwrap();

    if let Some(captures) = re.captures(link) {
        let username = captures.name("username").unwrap().as_str();
        let mut repo = captures.name("repo").unwrap().as_str().to_string();
        if repo.ends_with(".git") {
            repo = repo.replace(".git", "");
        }

        return (username.to_string(), repo.to_string());
    }

    ("".to_string(), "".to_string())
}

fn main() {
    let _pittacia: Pittacia = argh::from_env();

    println!("pittacia - Experimental CLI for managing GitHub issues label.");

    let selections = &[
        "Current directory as repository",
        "A remote repository or provide a repository URL",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select what type of repository you want to manage")
        .default(0)
        .items(&selections[..])
        .interact_opt()
        .unwrap();

    match selection {
        Some(0) => {
            println!("Current directory as repository");
        }
        Some(1) => {
            let github_link: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter the GitHub repository URL")
                .interact_text()
                .unwrap();

            let (username, repo) = extract_info_from_gh_link(&github_link);

            if username.is_empty() || repo.is_empty() {
                println!("Invalid GitHub repository URL");
                return;
            }

            println!("Username: {}", username);
            println!("Repository: {}", repo);
        }
        _ => {
            println!("No selection made");
        }
    }
}
