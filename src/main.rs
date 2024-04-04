mod github;

use argh::FromArgs;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use github::GitHub;

pub struct Labels {
    path: String,
    labels: Vec<String>,
}

pub struct Configuration {
    git: GitHub,
    labels: Labels,
}

#[derive(FromArgs)]
/// Experimental CLI for managing GitHub issue labels.
struct Pittacia {}

fn main() {
    let _pittacia: Pittacia = argh::from_env();
    let mut configuration = Configuration {
        git: GitHub {
            username: "".to_string(),
            repo: "".to_string(),
        },
        labels: Labels {
            path: "".to_string(),
            labels: vec![],
        },
    };

    println!("pittacia - Experimental CLI for managing GitHub issue labels.");

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
            let information = github::extract_from_local();

            match information {
                Ok(info) => {
                    configuration.git = info;
                    println!("{:?}", configuration.git);
                }
                Err(_) => {
                    println!("Could not extract information from the current directory.");
                }
            }
        }
        Some(1) => {
            let github_link: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter the GitHub repository URL")
                .interact_text()
                .unwrap();

            let info = github::extract_from_link(&github_link);

            match info {
                Ok(info) => {
                    configuration.git = info;
                    println!("{:?}", configuration.git);
                }
                Err(_) => {
                    println!("Could not extract information from the provided URL.");
                }
            }
        }
        _ => {
            print!("Invalid selection or no selection made.");
        }
    }
}
