use docopt_args::DocoptArgs;
use ProtonConfig;
use util;

/// Handles all "proton vixen ..." commands
pub fn handle_vixen(args: DocoptArgs, config: ProtonConfig) {
    if args.cmd_import_layout.unwrap_or(false) {
        vixen_import_layout(args, config)
    } else if args.cmd_import_sequence.unwrap_or(false) {
        vixen_import_sequence(args, config)
    } else {
        panic!("Docopt parsing failed!")
    }
}

// proton vixen import-layout <layout-file>
pub fn vixen_import_layout(args: DocoptArgs, config: ProtonConfig) {
    // Command: proton-vixen-converter.py import-layout <layout-file>

    let layout_path = format!("{}/Profiles/{}.pro", config.vixen_folder.clone(), args.arg_layout_file.unwrap());

    let args = [
        "import-layout".to_string(),
        layout_path,
        config.default_dmx_channel.to_string()];

    let _ = util::run_vixen_converter(&config.vixen_converter_py, &args);
}

// proton vixen import-sequence --seq=<seq-file> --audio=<audio-file> --layout-name=<layout-name>
pub fn vixen_import_sequence(args: DocoptArgs, config: ProtonConfig) {
    // Command: proton-vixen-converter.py import-sequence <admin-key> <seq-file> <audio-file> <layout-name>

    let sequence_path = config.vixen_folder.clone() + "/Sequences/" + &args.flag_seq.unwrap();
    let audio_path = config.vixen_folder + "/Audio/" + &args.flag_audio.unwrap();

    let args = [
        "import-sequence".to_string(),
        config.key,
        sequence_path,
        audio_path,
        args.flag_layout_name.unwrap().to_string()];

    let _ = util::run_vixen_converter(&config.vixen_converter_py, &args);
}

/*

converter.py import-sequence <admin-key> <seq-file> <audio-file> <layout-id>
converter.py import-layout <pro-file>
*/
