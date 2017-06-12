use std::process::{Command, Output};

pub fn run_proton_cli(command: &str, args: &[String]) -> Option<String> {
    let output = Command::new("proton_cli")
        .args(args)
        .output()
        .expect(&format!("Failed to run {}", command));

    handle_output(output)
}

pub fn run_vixen_converter(command: &str, args: &[String]) -> Option<String> {
    let output = Command::new("python3")
        .arg("../proton-vixen-converter/vixenconverter/converter.py")
        .args(args)
        .output()
        .expect(&format!("Failed to run {}", command));

    handle_output(output)
}

/// Return stdout and print stderr for commmand output. Return None on error
fn handle_output(output: Output) -> Option<String> {

    let out = String::from_utf8(output.stdout).unwrap_or(String::from("Invalid stdout"));
    let err = String::from_utf8(output.stderr).unwrap_or(String::from("Invalid stderr"));

    // Error occurred. Print error and exit
    if output.status.code().unwrap_or(-1) != 0 {
        println!("{}, {}", err, out);
        return None;
    }

    // No error - return output as String (None if invalid)
    // Also print output for fun
    println!("{}", out);
    Some(out)
}
