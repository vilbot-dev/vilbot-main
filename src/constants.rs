use once_cell::sync::Lazy;
use yaml_rust::Yaml;

use crate::utils::load_yaml_config;

pub static CONFIG: Lazy<Yaml> = Lazy::new(|| load_yaml_config("config.yaml"));
