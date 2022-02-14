use std::error::Error;
use std::path::Path;

use tokio::fs::File;
use tokio::io::AsyncReadExt;
use yaml_rust::{Yaml, YamlLoader};

pub struct Configuration(pub Vec<Yaml>);

impl Configuration {
    // Error implements the From trait, through blank implementation, to convert
    // from T to Box<T>
    pub async fn new(loc: &str) -> Result<Self, Box<dyn Error>> {
        let mut file = File::open(Path::new(loc)).await?;
        let mut config_str = String::new();
        file.read_to_string(&mut config_str).await?;
        let config = YamlLoader::load_from_str(&config_str)?;
        Ok(Self(config))
    }
}

#[cfg(test)]
mod test {

    use self::super::*;

    #[tokio::test]
    async fn test_configuration() {
        let config = Configuration::new("./configs/api.yml").await.unwrap();
        let config = &config.0[0];
        // Test API
        assert_eq!("api_test", config["api"]["test"].as_str().unwrap());
        // Test DB
        assert_eq!("db_test", config["db"]["test"].as_str().unwrap());
    }
}
