use docopt_args::DocoptArgs;
use ProtonConfig;
use std::io::{self, BufRead, stdout, StdinLock, Write};

pub fn initial_setup(_args: DocoptArgs) {
    configure(ProtonConfig::default_config())
}


pub fn configure(old_config: ProtonConfig) {
    // Setup IO stuff
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    // Get config values
    let key_path = get_config_value("key_path", old_config.key, &mut handle);
    let vixen_folder = get_config_value("vixen_folder", old_config.vixen_folder, &mut handle);
    let vixen_converter_py = get_config_value("vixen_converter_py", old_config.vixen_converter_py, &mut handle);
 
    // Create config file (overwrite if already there)
    let config = ProtonConfig::new(
        key_path.trim(),
        vixen_folder.trim(),
        vixen_converter_py.trim()
    );

    config.save()
}

// Gets a config value from user input. Uses default value if nothing is provided
fn get_config_value<'a>(prompt: &'a str, default: String, handle: &'a mut StdinLock) -> String {
    print!("{} [{}]: ", prompt, default);
    let _ = stdout().flush();
    let mut value = String::new();
    handle.read_line(&mut value).expect("Failed to read line of user input");
    value = value.trim().to_string();
    if value == "" {
        value = default;
    }

    value
}
