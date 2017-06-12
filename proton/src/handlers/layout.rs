use docopt_args::DocoptArgs;
use ProtonConfig;
use util;

/// Handles all "./proton layout ..." commands
pub fn handle_layout(args: DocoptArgs, config: ProtonConfig) {
    if args.cmd_new.unwrap_or(false) {
        handle_new(args, config)
    } else if args.cmd_get.unwrap_or(false) {
        handle_get(args, config)
    } else if args.cmd_delete.unwrap_or(false) {
        handle_delete(args, config)
    } else if args.cmd_update.unwrap_or(false) {
        handle_update(args, config)
    } else {
        panic!("Docopt parsing failed!");
    }
}

/// Handles the command "./proton layout new <layout-name> <layout-file>"
fn handle_new(args: DocoptArgs, config: ProtonConfig) {
    //#TODO: specify name of layout to proton_cli
    let command = "./proton_cli new-layout <layout-file>";
    let args = [
        "new-layout".to_string(),
        args.arg_layout_file.unwrap().to_string()];

    util::run_proton_cli(command, &args);
}

/// Handles the command "./proton layout get [<layout-name>]"
fn handle_get(args: DocoptArgs, config: ProtonConfig) {
    println!("./proton layout get [--id=<layout-name>]");
}

/// Handles the command "./proton layout delete [<layout-name>]"
fn handle_delete(args: DocoptArgs, config: ProtonConfig) {
    println!("./proton layout delete [--id=<layout-name>]");
}

/// Handles the command "./proton layout update <layout-name> --file=<layout-file>"
fn handle_update(args: DocoptArgs, config: ProtonConfig) {
    println!("./proton layout update --file=<layout-file> [--id=<layout-name>]");
}
