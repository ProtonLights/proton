use std::fs::File;
use std::io::Write;
use std::process::{Command, Output};

/// Runs proton_cli with the given command and arguments
pub fn run_proton_cli(command: &str, args: &[String]) -> Option<String> {
    let output = Command::new("proton_cli")
        .args(args)
        .output()
        .expect(&format!("Failed to run {}", command));

    handle_output(output)
}

/// Runs proton_vixen_converter.py with the given arguments
pub fn run_vixen_converter(args: &[String]) -> Option<String> {
    println!("Running proton_vixen_converter.py...");

    let output = Command::new("python3")
        .arg("../proton-vixen-converter/vixenconverter/converter.py")
        .args(args)
        .output()
        .expect(&"Failed to run proton_vixen_converter.py");

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

/// Writes an SSH key output to a file
pub fn write_key_to_file(key: String, file_name: &str) {
    // When handle_output returns an SSH key, it starts with a bunch of dashes
    let key_start = key.find("-").expect("Invalid output from proton_cli");

    // Write key
    let mut out_file = File::create(&file_name).expect("Failed to create key file");
    let _ = out_file.write(&key.into_bytes()[key_start..]);
}
