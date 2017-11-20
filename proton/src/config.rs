use serde_json;
use std::fs::File;
use std::io::{self, Read, Write};

#[derive(Serialize, Deserialize)]
pub struct Config {
    // Last auth key used (usually user key)
    pub key: String,
    // Points to "Sequence Data" Vixen folder
    pub vixen_folder: String,
    // Points to "vixenconverter/converter.py" from proton-vixen-converter
    pub vixen_converter_py: String
}

impl Config {
    // Creates a new, default config
    pub fn default_config() -> Config {
        return Config::new(
            "user.pub",
            "/home/lightshow/Dropbox/LightShow/Sequencing/Sequence Data",
            "/home/lightshow/git/proton/proton-vixen-converter/vixenconverter/converter.py");
    }

    // Creates new config file. Does not save
    pub fn new(
        key_path: &str,
        vixen_folder: &str,
        vixen_converter_py: &str
    ) -> Config {
    
        Config {
            key: key_path.to_string(),
            vixen_folder: vixen_folder.to_string(),
            vixen_converter_py: vixen_converter_py.to_string()
        }
    }

    // Loads config from file. If not found, creates new config at specified path
    pub fn load() -> Result<Config, io::Error> {
        // Open file
        let mut cfg_file = try!(File::open("proton.cfg"));

        // Read contents into string
        let mut contents = String::new();
        try!(cfg_file.read_to_string(&mut contents));

        // Decode and return
        let config: Config = serde_json::from_str(&contents).expect("Failed to decode config file from JSON");
        Ok(config)
    }

    // Saves config to file
    pub fn save(&self) {
        // Make file
        let mut cfg_file = File::create("proton.cfg").expect("Failed to create config file");

        // Serialize this object into a string
        let config = serde_json::to_string(&self).expect("Failed to encode config file into JSON");

        // Write to file
        cfg_file.write(&config.into_bytes()).expect("Failed to write config file to disk");
    }
}
