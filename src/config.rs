use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub extended_volume: bool,
}

impl Config {
    const DEFAULT: Self = Self {
        extended_volume: false,
    };

    pub fn new() -> Self {
        let config_path = xdg::BaseDirectories::new()
            .unwrap()
            .get_config_file("augment/vol-brt-man.toml");

        if !config_path.exists() {
            xdg::BaseDirectories::new()
                .unwrap()
                .create_config_directory("augment")
                .unwrap();

            std::fs::write(&config_path, toml::to_string(&Self::DEFAULT).unwrap()).unwrap();
        }

        toml::from_str(&std::fs::read_to_string(&config_path).unwrap()).unwrap()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
