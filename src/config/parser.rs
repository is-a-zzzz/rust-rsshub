use crate::error::Result;
use crate::config::types::{PluginConfig, SourceConfig};
use crate::error::RssHubError;
use std::path::Path;
use std::fs;
use std::time::SystemTime;

pub struct ConfigParser {
    configs_dir: String,
}

impl ConfigParser {
    pub fn new(configs_dir: impl Into<String>) -> Self {
        Self {
            configs_dir: configs_dir.into(),
        }
    }

    /// 检查插件配置文件是否存在
    pub fn plugin_exists(&self, name: &str) -> bool {
        let path = self.get_config_path(name);
        path.exists()
    }

    /// 获取配置文件路径
    pub fn get_config_path(&self, name: &str) -> std::path::PathBuf {
        Path::new(&self.configs_dir).join(format!("{}.yml", name))
    }

    /// 获取配置文件修改时间
    pub fn get_plugin_mtime(&self, name: &str) -> Result<SystemTime> {
        let path = self.get_config_path(name);
        let metadata = fs::metadata(&path)?;
        Ok(metadata.modified()?)
    }

    /// 加载并解析插件配置
    pub fn load_plugin(&self, name: &str) -> Result<PluginConfig> {
        let path = self.get_config_path(name);
        let content = fs::read_to_string(&path)?;

        // 解析 YAML
        let config: PluginConfig = serde_yaml::from_str(&content)?;

        // 验证配置
        self.validate_config(&config)?;

        Ok(config)
    }

    /// 验证配置有效性
    fn validate_config(&self, config: &PluginConfig) -> Result<()> {
        // 验证必需字段
        if config.feed.title.is_empty() {
            return Err(RssHubError::Config("Feed title cannot be empty".into()));
        }

        if config.feed.link.is_empty() {
            return Err(RssHubError::Config("Feed link cannot be empty".into()));
        }

        // 验证 URL 格式
        let url = match &config.source {
            SourceConfig::Html(cfg) => &cfg.url,
            SourceConfig::Json(cfg) => &cfg.url,
            SourceConfig::Xml(cfg) => &cfg.url,
        };

        if url.parse::<url::Url>().is_err() {
            return Err(RssHubError::InvalidUrl(url.clone()));
        }

        Ok(())
    }

    /// 列出所有可用的插件
    pub fn list_plugins(&self) -> Result<Vec<String>> {
        let dir = fs::read_dir(&self.configs_dir)?;
        let mut plugins = Vec::new();

        for entry in dir {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "yml" || ext == "yaml" {
                        if let Some(stem) = path.file_stem() {
                            if let Some(name) = stem.to_str() {
                                if !name.starts_with('.') {
                                    plugins.push(name.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }

        plugins.sort();
        Ok(plugins)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_path() {
        let parser = ConfigParser::new("/tmp/configs");
        let path = parser.get_config_path("test");
        assert_eq!(path, std::path::PathBuf::from("/tmp/configs/test.yml"));
    }
}
