use std::env;
use std::io;
use std::path;

fn get_current_directory() -> io::Result<path::PathBuf> {
    let current_dir = env::current_dir()?;
}

fn find_git_repo_root_recursively(root_path: &path::Path) -> io::Result<path::PathBuf> {
    let git_path = root_path.read_dir()?.find(
        |entry| {
            match entry {
                Ok(ref dir) => {
                    let path = dir.path();
                    if path.ends_with("/.git") {
                        true
                    }
                },
                _ => false,
            }
        });

    Ok(path::PathBuf::new())
}

mod git {
    use git2::Repository;
    use std::path::Path;

    struct Repo {
        repository: Repository,
    }

    impl Repo {
        fn open(path: &Path) -> Result<Self, git2::Error> {
            let repository = Repository::open(path)?;
            Ok(Repo {
                repository,
            })
        }
    }
}
