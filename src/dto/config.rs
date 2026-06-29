use serde::Deserialize;

use crate::dto::Command;
use crate::infra::atcoder::{page_type::Home, url::Url};
use std::path::{Path, PathBuf};
pub struct Config {
    pub app_config: AppConfig,
    pub user_config: UserConfig,
}

impl Config {
    pub fn new(project_root: &Path, app_config: AppConfig, user_config: UserConfig) -> Self {
        let app_config = app_config.with_project_root(project_root);
        let user_config = user_config.with_project_root(project_root);
        Self {
            app_config,
            user_config,
        }
    }
}

#[derive(Deserialize)]
pub struct AppConfig {
    pub path: PathConfig,
    pub url: UrlConfig,
}

impl AppConfig {
    fn with_project_root(self, project_root: &Path) -> Self {
        Self {
            path: self.path.with_project_root(project_root),
            url: self.url,
        }
    }
}

#[derive(Deserialize)]
pub struct PathConfig {
    pub session_data: PathBuf,
    pub tasks_info: PathBuf,
    pub test: PathBuf,
    pub user_config: PathBuf,
    pub metadata: PathBuf,
}

impl PathConfig {
    fn with_project_root(self, project_root: &Path) -> Self {
        Self {
            session_data: resolve_project_path(project_root, &self.session_data),
            tasks_info: resolve_project_path(project_root, &self.tasks_info),
            test: resolve_project_path(project_root, &self.test),
            user_config: resolve_project_path(project_root, &self.user_config),
            metadata: resolve_project_path(project_root, &self.metadata),
        }
    }
}

#[derive(Deserialize)]
pub struct UrlConfig {
    pub homepage: Url<Home>,
}

#[derive(Deserialize)]
pub struct UserConfig {
    language: Vec<LanguageConfig>,
}

impl UserConfig {
    pub fn language_config(&self, language: &str) -> Option<&LanguageConfig> {
        self.language.iter().find(|config| config.name == language)
    }

    fn with_project_root(self, project_root: &Path) -> Self {
        Self {
            language: self
                .language
                .into_iter()
                .map(|language| language.with_project_root(project_root))
                .collect(),
        }
    }
}

#[derive(Deserialize)]
pub struct LanguageConfig {
    name: String,
    id: String,
    src_path: PathBuf,
    pub compile: Option<Command>,
    pub execute: Command,
}

impl LanguageConfig {
    fn with_project_root(self, project_root: &Path) -> Self {
        Self {
            name: self.name,
            id: self.id,
            src_path: resolve_project_path(project_root, &self.src_path),
            compile: self
                .compile
                .map(|command| command.with_resolved_working_dir(project_root)),
            execute: self.execute.with_resolved_working_dir(project_root),
        }
    }
}

impl Command {
    fn with_resolved_working_dir(self, project_root: &Path) -> Self {
        Self {
            command: self.command,
            args: self.args,
            working_dir: resolve_project_path(project_root, &self.working_dir),
        }
    }
}

fn resolve_project_path(project_root: &Path, path: &Path) -> PathBuf {
    project_root.join(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_config_resolves_relative_app_paths_from_project_root() {
        let project_root = env::current_dir().unwrap().join("project");
        let config = Config::new(
            &project_root,
            AppConfig {
                path: PathConfig {
                    session_data: ".atcoder/session_data.json".into(),
                    tasks_info: ".atcoder/tasks_info.json".into(),
                    test: "test".into(),
                    user_config: ".atcoder/config.toml".into(),
                    metadata: ".atcoder".into(),
                },
                url: UrlConfig {
                    homepage: "https://atcoder.jp/home".to_string().into(),
                },
            },
            UserConfig { language: vec![] },
        );

        assert_eq!(
            project_root.join(".atcoder/session_data.json"),
            config.app_config.path.session_data
        );
        assert_eq!(
            project_root.join(".atcoder/tasks_info.json"),
            config.app_config.path.tasks_info
        );
        assert_eq!(project_root.join("test"), config.app_config.path.test);
        assert_eq!(
            project_root.join(".atcoder/config.toml"),
            config.app_config.path.user_config
        );
        assert_eq!(
            project_root.join(".atcoder"),
            config.app_config.path.metadata
        );
    }

    #[test]
    fn test_resolve_project_path_keeps_absolute_path() {
        let project_root = env::current_dir().unwrap().join("project");
        let path = project_root.join("absolute").join("path");

        assert_eq!(path, resolve_project_path(&project_root, &path));
    }

    #[test]
    fn test_user_config_resolves_command_working_dirs_from_project_root() {
        let project_root = env::current_dir().unwrap().join("project");
        let user_config = UserConfig {
            language: vec![LanguageConfig {
                name: "rust".to_string(),
                id: "5001".to_string(),
                src_path: "src/main.rs".into(),
                compile: Some(Command {
                    command: "cargo".to_string(),
                    args: vec!["build".to_string()],
                    working_dir: "workspace/rust".into(),
                }),
                execute: Command {
                    command: "cargo".to_string(),
                    args: vec!["run".to_string()],
                    working_dir: ".".into(),
                },
            }],
        }
        .with_project_root(&project_root);

        let language_config = user_config.language_config("rust").unwrap();
        assert_eq!(project_root.join("src/main.rs"), language_config.src_path);
        assert_eq!(
            project_root.join("workspace/rust"),
            language_config
                .compile
                .as_ref()
                .map(|command| command.working_dir.clone())
                .unwrap()
        );
        assert_eq!(project_root, language_config.execute.working_dir);
    }

    #[test]
    fn test_user_config_defaults_missing_command_working_dir_to_project_root() {
        let project_root = env::current_dir().unwrap().join("project");
        let user_config: UserConfig = toml::from_str(
            r#"
[[language]]
name = "rust"
id = "5001"
src_path = "src/main.rs"

[language.execute]
command = "cargo"
args = ["run"]
"#,
        )
        .unwrap();

        let user_config = user_config.with_project_root(&project_root);
        let language_config = user_config.language_config("rust").unwrap();

        assert_eq!(project_root, language_config.execute.working_dir);
    }
}
