use crate::{
    dto::config::{AppConfig, Config},
    infra::file_handler,
};
use std::{
    env,
    path::{Path, PathBuf},
};

pub fn load_config() -> Result<Config, ConfigLoadError> {
    // Load config for app
    let app_config = load_app_config()?;

    // Move to the root directory for app
    let metadata_path =
        find_required_in_ancestors(&app_config.path.metadata, ConfigLoadError::MetadataNotFound)?;
    let root_path =
        metadata_path
            .parent()
            .ok_or_else(|| ConfigLoadError::MetadataParentMissing {
                path: metadata_path.clone(),
            })?;
    env::set_current_dir(root_path).map_err(|source| ConfigLoadError::SetCurrentDir {
        source,
        path: root_path.to_path_buf(),
    })?;

    // Load config for user
    let user_config = file_handler::load_toml(&app_config.path.user_config)?;

    Ok(Config {
        app_config,
        user_config,
    })
}

fn load_app_config() -> Result<AppConfig, ConfigLoadError> {
    let app_config_path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/config.toml"));
    let app_config = toml::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/config.toml"
    )))
    .map_err(|source| ConfigLoadError::AppConfigSerde {
        message: source.to_string(),
        path: app_config_path.to_path_buf(),
    })?;
    Ok(app_config)
}

fn find_required_in_ancestors(
    target: &Path,
    not_found: fn(PathBuf) -> ConfigLoadError,
) -> Result<PathBuf, ConfigLoadError> {
    find_in_ancestors(target)?.ok_or_else(|| not_found(target.to_path_buf()))
}

#[cfg(test)]
fn find_required_in_ancestors_from(
    start: &Path,
    target: &Path,
    not_found: fn(PathBuf) -> ConfigLoadError,
) -> Result<PathBuf, ConfigLoadError> {
    find_in_ancestors_from(start, target).ok_or_else(|| not_found(target.to_path_buf()))
}

fn find_in_ancestors(target: &Path) -> Result<Option<PathBuf>, ConfigLoadError> {
    let current_dir = env::current_dir().map_err(ConfigLoadError::CurrentDir)?;
    Ok(find_in_ancestors_from(&current_dir, target))
}

fn find_in_ancestors_from(start: &Path, target: &Path) -> Option<PathBuf> {
    start
        .ancestors()
        .map(|path| path.join(target))
        .find(|path| path.is_dir())
}

#[derive(thiserror::Error, Debug)]
pub enum ConfigLoadError {
    #[error(transparent)]
    File(#[from] file_handler::Error),

    #[error("{message}: {path}")]
    AppConfigSerde { message: String, path: PathBuf },

    #[error("Failed to find {0} in current directory or ancestors")]
    MetadataNotFound(PathBuf),

    #[error("Failed to determine parent directory of metadata path: {path}")]
    MetadataParentMissing { path: PathBuf },

    #[error("Failed to get current directory")]
    CurrentDir(#[source] std::io::Error),

    #[error("{source}: {path}")]
    SetCurrentDir {
        #[source]
        source: std::io::Error,
        path: PathBuf,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_find_in_ancestors_from_found() {
        let root = unique_temp_dir("atcoder-tools-config-loader-found");
        let metadata = root.join(".atcoder-tools");
        let nested = root.join("a").join("b");
        fs::create_dir_all(&metadata).unwrap();
        fs::create_dir_all(&nested).unwrap();

        let result = find_in_ancestors_from(&nested, Path::new(".atcoder-tools"));

        assert_eq!(Some(metadata), result);

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn test_find_in_ancestors_from_not_found() {
        let root = unique_temp_dir("atcoder-tools-config-loader-not-found");
        let nested = root.join("a").join("b");
        fs::create_dir_all(&nested).unwrap();

        let result = find_in_ancestors_from(&nested, Path::new(".atcoder-tools"));

        assert_eq!(None, result);

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn test_find_required_in_ancestors_returns_metadata_not_found() {
        let root = unique_temp_dir("atcoder-tools-config-loader-required-not-found");
        let nested = root.join("a").join("b");
        fs::create_dir_all(&nested).unwrap();

        let error = find_required_in_ancestors_from(
            &nested,
            Path::new("missing-metadata"),
            ConfigLoadError::MetadataNotFound,
        )
        .unwrap_err();

        assert!(matches!(error, ConfigLoadError::MetadataNotFound(_)));

        fs::remove_dir_all(root).unwrap();
    }

    fn unique_temp_dir(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        env::temp_dir().join(format!("{name}-{nanos}"))
    }
}
