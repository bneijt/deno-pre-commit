// Mirroring script
// Takes the latest releases of the deno package and creates a tag for it
use regex::Regex;
use reqwest;
use serde_json;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // read jsonl list of metadata for deno
    let resp: Value = reqwest::get("https://registry.npmjs.org/deno")
        .await?
        .json()
        .await?;

    // We drop versions before 1.46.0 because there is no deno bin in those
    let versions: Vec<&String> = resp["versions"]
        .as_object()
        .unwrap()
        .keys()
        .skip_while(|&x| x != &"1.46.0")
        .collect::<Vec<_>>();

    let commit_hook_regex = Regex::new(r"(\s+- deno@).+").unwrap();
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

            let pre_commit_hooks = std::fs::read_to_string(".pre-commit-hooks.yaml")
                .expect("Failed to read .pre-commit-hooks.yaml");
            let new_pre_commit_hooks = commit_hook_regex
                .replace_all(&pre_commit_hooks, format!("${{1}}{}", version).as_str())
                .to_string();
            std::fs::write(".pre-commit-hooks.yaml", new_pre_commit_hooks)
                .expect("Failed to write .pre-commit-hooks.yaml");

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
