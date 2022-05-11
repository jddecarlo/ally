use std::env;
use std::io;
use std::path;

pub(crate) fn get_current_directory() -> io::Result<path::PathBuf> {
    env::current_dir()
}

pub(crate) mod git {
    use git2;
    use std::path;

    pub(crate) struct Repo {
        repository: git2::Repository,
    }

    impl Repo {
        pub(crate) fn discover(path: Option<&path::Path>) -> Result<Self, git2::Error> {
            let search_path = match path {
                Some(path) => path.to_path_buf(),
                None => super::get_current_directory().map_err(|e| git2::Error::from_str(&e.to_string()))?,
            };
            
            let repository = git2::Repository::discover(search_path)?;
            Ok(Repo {
                repository,
            })
        }

        pub(crate) fn print_incoming_revs(&self) -> Result<(), git2::Error> {
            let branch_name = self.current_branch_name()?;
            match branch_name {
                Some(branch_name) => println!("{}", branch_name),
                None => println!("Not on a named branch."),
            };
            Ok(())
        }

        fn current_branch_name(&self) -> Result<Option<String>, git2::Error> {
            let head_result = self.repository.head();
            match head_result {
                Ok(ref head_ref) => {
                    match head_ref.shorthand() {
                        Some(ref name) => Ok(Some(name.to_string())),
                        None => Ok(None),
                    }
                },
                Err(e) => {
                    match e.code() {
                        git2::ErrorCode::UnbornBranch | git2::ErrorCode::NotFound => Ok(None),
                        _ => return Err(e),
                    }
                },
            }
        }
    }
}
