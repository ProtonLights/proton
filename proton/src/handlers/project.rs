use docopt_args::DocoptArgs;
use ProtonConfig;
use util;

/// Handles all "proton project ..." commands
pub fn handle_project(args: DocoptArgs, config: ProtonConfig) {
    if args.cmd_new.unwrap_or(false) {
        project_new(args, config)
    } else if args.cmd_get.unwrap_or(false) {
        project_get(args, config)
    } else {
        panic!("Docopt parsing failed!")
    }
}

// proton project new <project-name> <layout-name>
pub fn project_new(args: DocoptArgs, mut config: ProtonConfig) {
    let command = "./proton_cli new-project <name> <layout-name>";
    let project_name = args.arg_project_name.unwrap();
    let args = [
        "new-project".to_string(),
        project_name.clone(),
        args.arg_layout_name.unwrap().to_string()];

    let output = util::run_proton_cli(command, &args);

    // Save admin key to file if successful
    if output.is_some() {
        let key = output.unwrap();
        let file_name = format!("{}.pub", project_name);
        util::write_key_to_file(key, &file_name);

        // Also add to config
        config.key = file_name;
        config.save();
    }
}

// proton project get <project-name>
pub fn project_get(args: DocoptArgs, config: ProtonConfig) {
    let command = "./proton_cli get-project <proj-name>";
    let args = [
        "get-project".to_string(),
        args.arg_project_name.unwrap()];

    let output = util::run_proton_cli(command, &args);
}

// proton project delete <project-name>
// pub fn project_delete(args: DocoptArgs, config: ProtonConfig) {
//     let command = "./proton_cli get-project <proj-name>";
//     let args = [
//         "get-project".to_string(),
//         args.arg_project_name.unwrap()];

//     util::run_proton_cli(command, &args);
// }

/*
  proton project new <project-name> <layout-name>
  proton project get <project-name>
  proton project delete <project-name>
  proton project add-sequence <seqid> [--index=<position>]
  proton project remove-sequence <position>
*/
