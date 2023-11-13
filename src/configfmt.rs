use serde::{de::DeserializeOwned, Serialize};
use std::io::BufRead;

#[derive(Default)]
pub enum ConfigFmt {
    #[cfg(feature = "yaml-cfg")]
    #[default]
    Yaml,
    #[cfg(feature = "json-cfg")]
    Json,
}

pub const CONFIG_NAMES: &[&str] = &[
    #[cfg(feature = "yaml-cfg")]
    "config.yaml",
    #[cfg(feature = "json-cfg")]
    "config.json",
];

impl ConfigFmt {
    #[inline]
    pub fn new(ext: &str) -> Option<Self> {
        match ext {
            #[cfg(feature = "yaml-cfg")]
            "yaml" => Some(ConfigFmt::Yaml),
            #[cfg(feature = "json-cfg")]
            "json" => Some(ConfigFmt::Json),
            _ => None,
        }
    }
    #[inline]
    pub fn from_reader<B: BufRead, T: DeserializeOwned>(&self, r: B) -> T {
        match self {
            #[cfg(feature = "yaml-cfg")]
            ConfigFmt::Yaml => serde_yaml::from_reader(r).unwrap(),
            #[cfg(feature = "json-cfg")]
            ConfigFmt::Json => serde_json::from_reader(r).unwrap(),
        }
    }
    #[inline]
    pub fn to_string<T: Serialize + ?Sized>(&self, src: &T) -> String {
        match self {
            #[cfg(feature = "yaml-cfg")]
            ConfigFmt::Yaml => serde_yaml::to_string(&src).unwrap(),
            #[cfg(feature = "json-cfg")]
            ConfigFmt::Json => serde_json::to_string_pretty(&src).unwrap(),
        }
    }
}


