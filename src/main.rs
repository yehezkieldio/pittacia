mod github;

use argh::FromArgs;
use dialoguer::{theme::ColorfulTheme, Input, Select};

#[derive(FromArgs)]
/// Experimental CLI for managing GitHub issue labels.
struct Pittacia {}

fn main() {
    let _pittacia: Pittacia = argh::from_env();

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
            let info = github::extract_from_local();
            println!("{:?}", info.unwrap());
        }
        Some(1) => {
            let github_link: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter the GitHub repository URL")
                .interact_text()
                .unwrap();

            let info = github::extract_from_link(&github_link);
            println!("{:?}", info.unwrap());
        }
        _ => {
            print!("Invalid selection or no selection made.");
        }
    }
}
