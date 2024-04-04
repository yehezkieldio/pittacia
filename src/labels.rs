use std::error::Error;

use indicatif::ProgressBar;
use octocrab::Octocrab;
use serde::{Deserialize, Serialize};

use crate::Configuration;

#[derive(Serialize, Deserialize, Debug)]
pub struct Label {
    pub name: String,
    pub color: String,
    pub description: String,
}

///
/// Extract labels from a JSON file in the data directory.
///
pub fn extract_default() -> Result<Vec<Label>, Box<dyn Error>> {
    let file = std::fs::read_to_string("data/labels.json")?;
    let labels: Vec<Label> = serde_json::from_str(&file)?;

    Ok(labels)
}

///
/// Extract labels from a JSON file in a specified path.
///
pub fn extract_from_path(path: &str) -> Result<Vec<Label>, Box<dyn Error>> {
    let file = std::fs::read_to_string(path)?;
    let labels: Vec<Label> = serde_json::from_str(&file)?;

    Ok(labels)
}

///
/// Get all labels from a GitHub repository.
///
#[allow(dead_code)]
pub async fn get_all_labels_from_repo(
    config: &Configuration,
    instance: Octocrab,
) -> octocrab::Result<()> {
    let username = &config.git.username;
    let repo = &config.git.repo;

    let labels = instance
        .issues(username, repo)
        .list_labels_for_repo()
        .send()
        .await?;

    for label in labels {
        println!("{:?}", label);
    }
    Ok(())
}

pub async fn remove_all_labels_from_repo(
    config: &Configuration,
    instance: &Octocrab,
) -> octocrab::Result<()> {
    let username = &config.git.username;
    let repo = &config.git.repo;

    let labels = instance
        .issues(username, repo)
        .list_labels_for_repo()
        .send()
        .await?;

    let bar = ProgressBar::new(labels.clone().into_iter().len() as u64);

    for label in labels {
        let encoded_name = urlencoding::encode(&label.name);

        instance
            .issues(username, repo)
            .delete_label(&encoded_name)
            .await?;

        bar.inc(1);
    }

    bar.finish();

    Ok(())
}

///
/// Append labels to a GitHub repository.
///
pub async fn append_to_repo(
    config: &Configuration,
    instance: &Octocrab,
) -> Result<(), Box<dyn Error>> {
    let bar = ProgressBar::new(config.labels.labels.len() as u64);

    for label in &config.labels.labels {
        let username = &config.git.username;
        let repo = &config.git.repo;

        create_label(instance.clone(), username, repo, label).await?;

        bar.inc(1);
    }

    bar.finish();
    Ok(())
}

///
/// Create a label in a GitHub repository.
///
async fn create_label(
    instance: Octocrab,
    username: &str,
    repo: &str,
    label: &Label,
) -> octocrab::Result<()> {
    instance
        .issues(username, repo)
        .create_label(&label.name, &label.color, &label.description)
        .await?;

    Ok(())
}
