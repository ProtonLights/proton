use docopt_args::DocoptArgs;
use ProtonConfig;
use util;
use std::fs;


pub fn handle_user(args: DocoptArgs, config: ProtonConfig) {
    if args.cmd_add.unwrap_or(false) {
        user_add(args, config)
    } else if args.cmd_delete.unwrap_or(false) {
        user_delete(args, config)
    } else if args.cmd_get.unwrap_or(false) {
        user_get(args, config)
    } else {
        panic!("Docopt parsing failed!")
    }
}

// proton user add <user-name>
fn user_add(args: DocoptArgs, config: ProtonConfig) {
    let command = "./proton_cli new-user <admin-key> <name>";
    let user_name = args.arg_user_name.unwrap();
    let args = [
        "new-user".to_string(),
        config.key,
        user_name.clone()];

    let output = util::run_proton_cli(command, &args);

    // Save user key to file if successful
    if output.is_some() {
        let key = output.unwrap();
        let file_name = format!("{}.pub", user_name.replace(" ", ""));
        util::write_key_to_file(key, &file_name);

        // Tell the user where their key file is
        println!("User creation successful. Key saved to {}. DO NOT share this key!", file_name);
    }
}
// proton user delete <user-name>
fn user_delete(args: DocoptArgs, config: ProtonConfig) {
    let command = "./proton_cli remove-user <admin-key> <name>";
    let user_name = args.arg_user_name.unwrap();
    let args = [
        "remove-user".to_string(),
        config.key,
        user_name.clone()];

    let output = util::run_proton_cli(command, &args);

    // Delete key file if command successful
    if output.is_some() {
        let file_name = format!("{}.pub", user_name.replace(" ", ""));
        fs::remove_file(&file_name).expect("Failed to remove user key file");
    }
}

// Eventually: proton user get <user-name>
// Now: proton user get <user-key>
fn user_get(args: DocoptArgs, config: ProtonConfig) {
    let command = "./proton_cli get-user <public-key>";
    let args = [
        "get-user".to_string(),
        args.arg_user_key.unwrap()];

    let _ = util::run_proton_cli(command, &args);
}
