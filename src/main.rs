// Mirroring script
// Takes the latest releases of the deno package and creates a tag for it
//
use regex::Regex;
use reqwest;
use serde_json;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // read jsonl list of metadata for deno
    let resp = reqwest::get("https://index.crates.io/de/no/deno")
        .await?
        .text()
        .await?;
    // for each line in the jsonl file, parse json if possible and take vers attribute of the object
    let mut versions = Vec::new();
    for line in resp.lines() {
        let version_metadata: Value = serde_json::from_str(line)?;
        let vers = version_metadata["vers"].as_str().unwrap().to_string();
        versions.push(vers.clone());
    }

    let commit_hook_regex = Regex::new(r"(\s+- deno@).+").unwrap();
    let readme_regex = Regex::new(r"(\s+rev: ).+").unwrap();

    for version in versions {
        //Only clean versions
        if version.contains("-") {
            continue;
        }
        // Check if the git tag for the version already exists
        let git_tag = format!("refs/tags/{}", version);
        let git_tag_exists = std::process::Command::new("git")
            .arg("show-ref")
            .arg("--verify")
            .arg("--quiet")
            .arg(git_tag)
            .status()
            .expect("failed to execute process")
            .success();

        println!("Is {} already a tag? {}", version, git_tag_exists);
        if !git_tag_exists {
            // Update the README.md file and the .pre-commit-hooks.yaml file
            let readme = std::fs::read_to_string("README.md").expect("Failed to read README.md");
            let new_readme = readme_regex
                .replace_all(&readme, format!("${{1}}{}", version).as_str())
                .to_string();
            std::fs::write("README.md", new_readme).expect("Failed to write README.md");

            let pre_commit_hooks = std::fs::read_to_string(".pre-commit-hooks.yaml")
                .expect("Failed to read .pre-commit-hooks.yaml");
            let new_pre_commit_hooks = commit_hook_regex
                .replace_all(&pre_commit_hooks, format!("${{1}}{}", version).as_str())
                .to_string();
            std::fs::write(".pre-commit-hooks.yaml", new_pre_commit_hooks)
                .expect("Failed to write .pre-commit-hooks.yaml");

            // Commit the changes
            println!("Committing changes for version {}", version);
            std::process::Command::new("git")
                .arg("add")
                .arg("README.md")
                .arg(".pre-commit-hooks.yaml")
                .status()
                .expect("failed to execute process");
            std::process::Command::new("git")
                .arg("commit")
                .arg("-m")
                .arg(format!("Update to deno@{}", version))
                .status()
                .expect("failed to execute process");
            std::process::Command::new("git")
                .arg("tag")
                .arg(version)
                .status()
                .expect("failed to execute process");
        }
    }
    Ok(())
}
