use std::fs;
use std::path::Path;
use serde::Deserialize;
use serde_json;
use text_io::read;
use crate::*;
use crate::utilities::git;

pub trait Executable<T> {
    fn execute(&self) -> AllyResult<T>;
}

pub struct FixPathSeparatorsCommand {
    pub path: Option<String>,
}

impl FixPathSeparatorsCommand {
    pub fn new(path: Option<String>) -> Self {
        Self { path }
    }
}

impl Executable<()> for FixPathSeparatorsCommand {
    fn execute(&self) -> AllyResult<()> {
        let input = match &self.path {
            Some(ref path) => path.to_owned(),
            None => read!("{}\0"),
        };

        let result;
        if cfg!(windows) {
            result = input.replace("/", "\\");
        } else {
            result = input.replace("\\", "/");
        }

        println!("{result}");
        Ok(())
    }
}

pub struct GitIncomingCommand { }

impl GitIncomingCommand {
    pub fn new() -> Self {
        Self { }
    }
}

impl Executable<()> for GitIncomingCommand {
    fn execute(&self) -> AllyResult<()> {
        git::print_incoming_commits()?;
        Ok(())
    }
}

pub struct GitOutgoingCommand { }

impl GitOutgoingCommand {
    pub fn new() -> Self {
        Self { }
    }
}

impl Executable<()> for GitOutgoingCommand {
    fn execute(&self) -> AllyResult<()> {
        git::print_outgoing_commits()?;
        Ok(())
    }
}

#[derive(Deserialize, Clone)]
struct AliasConfig {
    name: String,
    scope: String,
    kind: String,
    args: String,
}

impl From<git::Alias> for AliasConfig {
    fn from(alias: git::Alias) -> Self {
        let name = alias.name.clone();
        let scope = match &alias.scope {
            git::AliasScope::Global => "global".to_string(),
            git::AliasScope::System => "system".to_string(),
            git::AliasScope::Local => "local".to_string(),
            git::AliasScope::Worktree => "worktree".to_string(),
        };
        let (kind, args) = match &alias.info {
            git::AliasInfo::GitCommand(s) => {
                ("git".to_string(), s.clone())
            },
            git::AliasInfo::PythonCommand(s) => {
                ("python".to_string(), s.clone())
            },
            git::AliasInfo::AliasCommand(s) => {
                ("alias".to_string(), s.clone())
            },
        };
        
        AliasConfig {
            name,
            scope,
            kind,
            args,
        }
    }
}

impl Into<git::Alias> for AliasConfig {
    fn into(self) -> git::Alias {
        let scope = match &self.scope[..] {
            "global" => git::AliasScope::Global,
            "system" => git::AliasScope::System,
            "local" => git::AliasScope::Local,
            "worktree" => git::AliasScope::Worktree,
            _ => panic!("Unknown alias scope: {}", self.scope),
        };

        let info = match &self.kind[..] {
            "git" => git::AliasInfo::GitCommand(self.args),
            "python" => git::AliasInfo::PythonCommand(self.args),
            "alias" => git::AliasInfo::AliasCommand(self.args),
            _ => panic!("Unknown alias kind: {}", self.kind),
        };

        git::Alias::new(scope, self.name, info)
    }
}

#[derive(Deserialize)]
struct GitConfig {
    aliases: Vec<AliasConfig>,
}

pub struct EnvironmentCommand {
    pub path: Option<String>,
}

impl EnvironmentCommand {
    pub fn new(path: Option<String>) -> Self {
        Self { path }
    }
}

impl Executable<()> for EnvironmentCommand {
    fn execute(&self) -> AllyResult<()> {
        let base_path: &Path = Path::new(match &self.path {
            Some(ref path) => &path[..],
            None => ".",
        });
        let git_config_path = base_path.join("git_config.json");
        let git_config_string = fs::read_to_string(git_config_path)?;

        let git_config : GitConfig = serde_json::from_str(&git_config_string)?;
        git_config.aliases.iter().for_each(|alias| {
            let git_alias : git::Alias = alias.clone().into();
            let _ = git::add_git_alias(&git_alias);
        });

        Ok(())
    }
}