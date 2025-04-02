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

    // Get the relative path from repo root
    let relative_path = match get_relative_path_from_repo_root(&file_path) {
        Some(path) => path,
        None => file_path
            .as_ref()
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string(),
    };

    // Read current content
    let exclude_content = std::fs::read_to_string(&exclude_path)?;
    let lines: Vec<&str> = exclude_content.lines().collect();

    // Only add if it doesn't already exist
    if lines.contains(&relative_path.as_str()) {
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

    // Add the file path
    writeln!(exclude_file, "{}", relative_path)?;
    println!(
        "Added {} to local git exclude file (.git/info/exclude)",
        relative_path
    );

    Ok(())
}

/// Gets the path to the repository root
///
/// # Returns
///
/// * `String` - Path to the repository root
pub fn get_repo_root() -> String {
    let output = std::process::Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .expect("Failed to execute git command to find repo root");

    if !output.status.success() {
        panic!("Failed to get git repository root");
    }

    String::from_utf8(output.stdout)
        .expect("Invalid UTF-8 in git repo path")
        .trim()
        .to_string()
}

/// Gets the path to a file relative to repository root
///
/// # Arguments
///
/// * `file_path` - Path to the file
///
/// # Returns
///
/// * `Option<String>` - Relative path if successful, None otherwise
fn get_relative_path_from_repo_root<P: AsRef<std::path::Path>>(file_path: P) -> Option<String> {
    let repo_root = get_repo_root();
    let repo_root_path = std::path::Path::new(&repo_root);

    let absolute_path = if file_path.as_ref().is_absolute() {
        file_path.as_ref().to_path_buf()
    } else {
        std::env::current_dir().ok()?.join(file_path.as_ref())
    };

    absolute_path
        .strip_prefix(repo_root_path)
        .ok()
        .map(|rel_path| rel_path.to_string_lossy().to_string())
}
