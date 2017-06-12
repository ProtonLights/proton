use rustc_serialize::json;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(RustcEncodable, RustcDecodable)]
pub struct Config {
    cfg_path: String,
    pub key: String,
    pub vixen_folder: String,
}

impl Config {
    pub fn new(config_path: String) -> Config {
        Config {
            cfg_path: config_path,
            key: "key.pem".to_string(),
            vixen_folder: Path::new("/home/ryan/Dropbox/Great Northern Sequencing/Sequencing/Sequence Data")
                .to_str().expect("Vixen folder path not valid unicode").to_owned(),
        }
    }

    // Saves config to file
    pub fn save(&self) {
        let mut cfg_file = File::create(&self.cfg_path).expect("Failed to create config file");
        let config = json::encode(&self).expect("Failed to encode config file into JSON");
        cfg_file.write(&config.into_bytes());
    }
}
