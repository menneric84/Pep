use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::editor::Action;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub keys: Keys,
    pub theme: String,
    #[serde(default)]
    pub plugins: HashMap<String, String>,
    pub log_file: Option<String>,
    pub mouse_scroll_lines: Option<usize>,
    #[serde(default = "default_true")]
    pub show_diagnostics: bool,
}

impl Config {
    pub fn path(p: &str) -> PathBuf {
        #[allow(deprecated)]
        std::env::home_dir()
            .unwrap()
            .join("repos/pep/src/config/")
            .join(p)
    }
}

pub fn default_true() -> bool {
    true
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum KeyAction {
    Single(Action),
    Multiple(Vec<Action>),
    Nested(HashMap<String, KeyAction>),
    Repeating(u16, Box<KeyAction>),
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Keys {
    #[serde(default)]
    pub normal: HashMap<String, KeyAction>,
    #[serde(default)]
    pub insert: HashMap<String, KeyAction>,
    #[serde(default)]
    pub command: HashMap<String, KeyAction>,
}

