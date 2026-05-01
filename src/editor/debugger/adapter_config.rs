use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Deserialize)]
pub struct AdapterConfig {
    pub id: String,
    pub display_name: String,
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub file_extensions: Vec<String>,
    #[serde(default)]
    pub dap_adapter_type: String,
    #[serde(default)]
    pub dap_transport: String,
    #[serde(default)]
    pub session_cwd_template: String,
    #[serde(default = "default_health_check_args")]
    pub health_check_args: Vec<String>,
    #[serde(default = "default_client_addr_arg")]
    pub client_addr_arg: String,
    #[serde(default)]
    pub launch_template: Value,
    #[serde(default)]
    pub launch_overrides: Value,
}

fn default_health_check_args() -> Vec<String> {
    vec!["--version".to_string()]
}

fn default_client_addr_arg() -> String {
    "--client-addr".to_string()
}

#[derive(Debug)]
pub enum AdapterConfigError {
    NotFound,
    Io(std::io::Error),
    Parse(toml::de::Error),
}

pub fn discover_adapter_configs() -> Vec<AdapterConfig> {
    let mut map: HashMap<String, AdapterConfig> = HashMap::new();

    if let Ok(mut config_dir) = get_config_dir() {
        config_dir.push("debuggers");
        load_from_dir(&config_dir, &mut map, false);
    }

    let default_dir = Path::new("docs/examples/default/debuggers");
    load_from_dir(default_dir, &mut map, true);

    map.into_values().collect()
}

fn load_from_dir(dir: &Path, map: &mut HashMap<String, AdapterConfig>, is_default: bool) {
    let Ok(read_dir) = fs::read_dir(dir) else {
        return;
    };

    for entry in read_dir {
        let Ok(entry) = entry else { continue };
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("toml") {
            continue;
        }

        let Ok(cfg) = load_adapter_config(&path) else {
            continue;
        };

        if is_default
            && let Some(existing) = map.get_mut(&cfg.id)
        {
            merge_missing_fields_from_default(existing, &cfg);
            continue;
        }
        map.insert(cfg.id.clone(), cfg);
    }
}

fn merge_missing_fields_from_default(current: &mut AdapterConfig, default_cfg: &AdapterConfig) {
    if current.display_name.is_empty() {
        current.display_name = default_cfg.display_name.clone();
    }
    if current.command.is_empty() {
        current.command = default_cfg.command.clone();
    }
    if current.args.is_empty() {
        current.args = default_cfg.args.clone();
    }
    if current.file_extensions.is_empty() {
        current.file_extensions = default_cfg.file_extensions.clone();
    }
    if current.dap_adapter_type.is_empty() {
        current.dap_adapter_type = default_cfg.dap_adapter_type.clone();
    }
    if current.dap_transport.is_empty() {
        current.dap_transport = default_cfg.dap_transport.clone();
    }
    if current.session_cwd_template.is_empty() {
        current.session_cwd_template = default_cfg.session_cwd_template.clone();
    }
    if current.health_check_args == default_health_check_args() && !default_cfg.health_check_args.is_empty() {
        current.health_check_args = default_cfg.health_check_args.clone();
    }
    if current.client_addr_arg == default_client_addr_arg() && !default_cfg.client_addr_arg.is_empty() {
        current.client_addr_arg = default_cfg.client_addr_arg.clone();
    }
    if current.launch_template.is_null() {
        current.launch_template = default_cfg.launch_template.clone();
    }
}

pub fn load_adapter_config(path: &Path) -> Result<AdapterConfig, AdapterConfigError> {
    if !path.exists() {
        return Err(AdapterConfigError::NotFound);
    }
    let contents = fs::read_to_string(path).map_err(AdapterConfigError::Io)?;
    toml::from_str(&contents).map_err(AdapterConfigError::Parse)
}

fn get_config_dir() -> Result<PathBuf, AdapterConfigError> {
    #[cfg(target_os = "windows")]
    {
        std::env::var("APPDATA")
            .map(|appdata| PathBuf::from(appdata).join("den"))
            .map_err(|_| AdapterConfigError::NotFound)
    }

    #[cfg(not(target_os = "windows"))]
    {
        std::env::var("HOME")
            .map(|home| PathBuf::from(home).join(".config").join("den"))
            .map_err(|_| AdapterConfigError::NotFound)
    }
}
