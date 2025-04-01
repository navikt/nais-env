// Git utility functions

/// Checks if the current directory is inside a git repository
///
/// # Returns
///
/// * `bool` - True if inside a git repository, false otherwise
pub fn is_in_git_repo() -> bool {
    std::process::Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Gets the path to the git directory (.git)
///
/// # Returns
///
/// * `Option<String>` - Path to the git directory if successful, None otherwise
pub fn get_git_dir() -> Option<String> {
    std::process::Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| s.trim().to_string())
}

/// Gets the path to git's exclude file
///
/// # Returns
///
/// * `Option<std::path::PathBuf>` - Path to the exclude file if in a git repo, None otherwise
pub fn get_git_exclude_path() -> Option<std::path::PathBuf> {
    get_git_dir().map(|dir| std::path::Path::new(&dir).join("info/exclude"))
}

/// Adds a file to git's exclude list
///
/// # Arguments
///
/// * `file_path` - Path to the file to exclude
///
/// # Returns
///
/// * `io::Result<()>` - Success or error
pub fn add_to_git_exclude<P: AsRef<std::path::Path>>(file_path: P) -> std::io::Result<()> {
    if !is_in_git_repo() {
        return Ok(());
    }

    let exclude_path = match get_git_exclude_path() {
        Some(path) if path.exists() => path,
        _ => return Ok(()),
    };

    let file_name = file_path
        .as_ref()
        .file_name()
        .unwrap_or_default()
        .to_string_lossy();

    // Read current content
    let exclude_content = std::fs::read_to_string(&exclude_path)?;
    let lines: Vec<&str> = exclude_content.lines().collect();

    // Only add if it doesn't already exist
    if lines.contains(&file_name.as_ref()) {
        return Ok(());
    }

    // Prepare for adding the entry
    let mut exclude_file = std::fs::OpenOptions::new()
        .append(true)
        .open(&exclude_path)?;

    use std::io::Write;

    // Add the comment if needed
    if !lines.contains(&"# Added by nais-env") {
        writeln!(exclude_file, "# Added by nais-env")?;
    }

    // Add the file name
    writeln!(exclude_file, "{}", file_name)?;
    println!(
        "Added {} to local git exclude file (.git/info/exclude)",
        file_name
    );

    Ok(())
}
