use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ArborConfig {
    #[serde(rename = "worktree-path")]
    pub worktree_path: String,
}

impl Default for ArborConfig {
    fn default() -> Self {
        Self {
            worktree_path: "{repo}.{branch}".to_string(),
        }
    }
}

pub fn load_config() -> Result<ArborConfig, confy::ConfyError> {
    confy::load("arbor", None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ArborConfig::default();
        assert_eq!(config.worktree_path, "{repo}.{branch}");
    }

    #[test]
    fn test_config_serialization() {
        let config = ArborConfig::default();
        let toml = toml::to_string(&config).unwrap();
        assert!(toml.contains("worktree-path"));
        assert!(toml.contains("{repo}.{branch}"));
    }
}
