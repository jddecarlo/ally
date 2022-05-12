use std::io;
use crate::utilities::shell;

#[derive(Debug)]
struct BranchPair {
    local_branch: String,
    remote_branch: String,
}

impl BranchPair {
    fn new(local_branch: String, remote_branch: String) -> BranchPair {
        BranchPair {
            local_branch,
            remote_branch,
        }
    }
}

pub(crate) fn print_incoming_commits() -> io::Result<()> {
    let branches = get_current_branch_pair();
    if branches.is_none() {
        println!("Failed to get branch information.");
        return Ok(());
    }

    let branches = branches.unwrap();
    
    fetch()?;

    let diff_output = get_diff_commits_between_branches(&branches.local_branch, &branches.remote_branch)?;
    if diff_output.is_empty() {
        println!("No incoming commits.");
    } else {
        println!("{}", diff_output);
    }
    
    Ok(())
}

fn fetch() -> io::Result<()> {
    let args = vec!["fetch"];
    let fetch_result = shell::execute_shell_command("git", &args[..], None)?;

    if fetch_result.status.success() {
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Failed to fetch."))
    }
}

fn get_current_branch_pair() -> Option<BranchPair> {
    fn get_branch_name(command: &str, args: &[&str]) -> Option<String> {
        let current_branch_result = shell::execute_shell_command(command, args, None);
        if current_branch_result.is_err() {
            return None;
        }
    
        let current_branch_output = current_branch_result.unwrap();
        if !current_branch_output.status.success() {
            return None;
        }

        let current_branch_name_result = String::from_utf8(current_branch_output.stdout);
        if current_branch_name_result.is_err() {
            return None;
        }

        Some(current_branch_name_result.unwrap())
    }

    let args = vec!["rev-parse", "--abbrev-ref", "--symbolic-full-name", "HEAD"];
    let local_branch = get_branch_name("git", &args[..]);
    let args = vec!["rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{upstream}"];
    let remote_branch = get_branch_name("git", &args[..]);
    if local_branch.is_none() || remote_branch.is_none() {
        None
    } else {
        Some(BranchPair::new(local_branch.unwrap().trim().to_string(), remote_branch.unwrap().trim().to_string()))
    }
}

fn get_diff_commits_between_branches(from_branch_name: &str, to_branch_name: &str) -> io::Result<String> {
    let range = format!("{}..{}", from_branch_name, to_branch_name);
    let args = vec!["log", "--no-merges", &range];
    let log_result = shell::execute_shell_command("git", &args[..], None)?;

    if log_result.status.success() {
        Ok(String::from_utf8(log_result.stdout).map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to get commits."))?)
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Failed to get commits."))
    }
}
