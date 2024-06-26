use std::{path::PathBuf, sync::OnceLock};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct SomnuscapeConfig {
    pub model_temperature: f32,
    pub tone_words: Vec<String>,
    pub tone_words_per_generation: usize,
}

impl Default for SomnuscapeConfig {
    fn default() -> Self {
        Self {
            model_temperature: 0.9,
            tone_words: vec![
                "mystical".into(),
                "ancient".into(),
                "dark".into(),
                "light".into(),
                "gothic".into(),
                "holy".into(),
            ],
            tone_words_per_generation: 2,
        }
    }
}

pub fn get() -> &'static SomnuscapeConfig {
    static CONFIG: OnceLock<SomnuscapeConfig> = OnceLock::new();

    CONFIG.get_or_init(|| {
        let p: PathBuf = "config.yaml".into();
        if p.try_exists().unwrap_or_default() {
            std::fs::read_to_string(p)
                .and_then(|y| Ok(serde_yaml::from_str(&y)))
                .expect("Could not read config")
                .expect("Could not deserialize config")
        } else {
            SomnuscapeConfig::default()
        }
    })
}
