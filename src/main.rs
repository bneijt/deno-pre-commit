// Mirroring script
// Takes the latest releases of the deno package and creates a tag for it
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
        // Ignore yanked versions
        if version_metadata["yanked"].as_bool().unwrap() {
            continue;
        }
        let vers = version_metadata["vers"].as_str().unwrap().to_string();
        //Only clean versions, no -alpha or -pre
        if vers.contains("-") {
            continue;
        }
        versions.push(vers.clone());
    }

    let readme_regex = Regex::new(r"(\s+rev: ).+").unwrap();

    for version in versions {
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

        println!("Is '{}' already a tag? {}", version, git_tag_exists);
        if !git_tag_exists {
            // Update the README.md file and the .pre-commit-hooks.yaml file
            let readme = std::fs::read_to_string("README.md").expect("Failed to read README.md");
            let new_readme = readme_regex
                .replace_all(&readme, format!("${{1}}{}", version).as_str())
                .to_string();
            std::fs::write("README.md", new_readme).expect("Failed to write README.md");

            //Run npm to force a version
            std::process::Command::new("npm")
                .arg("install")
                .arg("--save")
                .arg("--save-exact")
                .arg(format!("deno@{}", version))
                .status()
                .expect("failed to execute process");

            // Commit the changes
            println!("Committing changes for version '{}'", version);
            std::process::Command::new("git")
                .arg("add")
                .arg("--all")
                .status()
                .expect("failed to execute process");
            std::process::Command::new("git")
                .arg("commit")
                .arg("--no-verify")
                .arg("-m")
                .arg(format!("Use deno@{}", version))
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
