use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use yaml_rust::{Yaml, YamlLoader};

pub struct Config(pub Vec<Yaml>);

impl Config {
    // Error implements the From trait, through blank implementation, to convert
    // from T to Box<T>
    pub fn new(loc: &str) -> Result<Self, Box<dyn Error>> {
        let mut file = File::open(Path::new(loc))?;
        let mut config_str = String::new();
        file.read_to_string(&mut config_str)?;
        let config = YamlLoader::load_from_str(&config_str)?;
        Ok(Self(config))
    }
}

#[cfg(test)]
mod test {

    use self::super::*;

    #[test]
    fn test_configuration() {
        let config = Config::new("./configs/api.yml").unwrap();
        let config = &config.0[0];
        // Test API
        assert_eq!("api_test", config["api"]["test"].as_str().unwrap());
        // Test DB
        assert_eq!("db_test", config["db"]["test"].as_str().unwrap());
    }
}
