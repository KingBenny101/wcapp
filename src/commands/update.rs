use anyhow::Result;
use self_github_update_enhanced::backends::github::Update;

/// Update wcapp to the latest version
pub fn execute() -> Result<()> {
    // Use self-github-update-enhanced for self-update
    let status = Update::configure()
        .repo_owner("KingBenny101")
        .repo_name("wcapp")
        .bin_name("wcapp")
        .show_download_progress(true)
        .show_output(true)
        .current_version(env!("CARGO_PKG_VERSION"))
        .build()?
        .update()?;

    if status.updated() {
        println!("wcapp was updated to version {}!", status.version());
    } else {
        println!("No update was performed. You are already running the latest version.");
    }
    Ok(())
}
