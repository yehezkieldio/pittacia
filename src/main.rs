mod github;
mod labels;

use argh::FromArgs;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use github::GitHub;
use labels::Label;
use octocrab::Octocrab;

#[derive(Debug)]
pub struct Labels {
    labels: Vec<Label>,
}

#[derive(Debug)]
pub struct Options {
    override_existing: bool,
}

#[derive(Debug)]
pub struct Configuration {
    token: String,
    git: GitHub,
    labels: Labels,
    opts: Options,
}

// async fn test_label(instance: Octocrab) -> octocrab::Result<()> {
//     let create_label = instance
//         .issues("yehezkieldio", "CollegeVisualBasic")
//         .create_label("test label", "fff", "test label")
//         .await?;

//     println!("{:?}", create_label);

//     Ok(())
// }

#[derive(FromArgs)]
/// Experimental CLI for managing GitHub issue labels.
struct Pittacia {}

#[tokio::main]
async fn main() {
    let _pittacia: Pittacia = argh::from_env();
    let mut configuration = Configuration {
        git: GitHub {
            username: "".to_string(),
            repo: "".to_string(),
        },
        labels: Labels { labels: vec![] },
        opts: Options {
            override_existing: false,
        },
        token: "".to_string(),
    };

    println!("pittacia - Experimental CLI for managing GitHub issue labels.");

    let token: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter your GitHub personal access token")
        .interact_text()
        .unwrap();

    configuration.token = token;

    let octocrab = Octocrab::builder()
        .personal_token(configuration.token.clone())
        .build()
        .unwrap();

    match octocrab.repos("yehezkieldio", "pittacia").get().await {
        Ok(_) => {
            println!("GitHub personal access token is valid.");
        }
        Err(_) => {
            println!("Please provide a valid GitHub personal access token.");

            return;
        }
    }

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

    let selections = &[
        "Use a default set of labels",
        "Provide a path to a JSON file",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select how you want to manage labels")
        .default(0)
        .items(&selections[..])
        .interact_opt()
        .unwrap();

    match selection {
        Some(0) => {
            let labels = labels::extract_default();

            match labels {
                Ok(labels) => {
                    configuration.labels.labels = labels;
                }
                Err(_) => {
                    println!("Could not extract labels from the default file.");
                }
            }
        }
        Some(1) => {
            let path: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter the path to the JSON file")
                .interact_text()
                .unwrap();

            let labels = labels::extract_from_path(&path);

            match labels {
                Ok(labels) => {
                    configuration.labels.labels = labels;
                }
                Err(_) => {
                    println!("Could not extract labels from the provided path.");
                }
            }
        }
        _ => {
            print!("Invalid selection or no selection made.");
        }
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Override existing labels?")
        .default(0)
        .items(&["Yes", "No"])
        .interact_opt()
        .unwrap();

    match selection {
        Some(0) => {
            configuration.opts.override_existing = true;
        }
        Some(1) => {
            configuration.opts.override_existing = false;
        }
        _ => {
            print!("Invalid selection or no selection made.");
        }
    }

    if configuration.opts.override_existing {
        println!("Removing all labels from the repository.");

        labels::remove_all_labels_from_repo(&configuration, &octocrab)
            .await
            .unwrap();
    }

    println!("Creating labels in the repository.");

    labels::append_to_repo(&configuration, &octocrab)
        .await
        .unwrap();

    println!("Labels have been successfully created.");
}
