
pub const USAGE: &'static str = "
Command-line interface for Proton

Usage:
  proton layout new <layout-name> <layout-file>
  proton layout get [<layout-name>]
  proton layout delete [<layout-name>]
  proton layout update [<layout-name>] --file=<layout-file>
  proton project new <project-name> <layout-name>
  proton project get <project-name>
  proton project delete <project-name>
  proton project add-sequence <seqid> [--index=<position>]
  proton project remove-sequence <position>
  proton section new --start=<tstart> --end=<tend> --sequence=<seqid> --fixtures=<fixids>
  proton section get <secid>
  proton section delete <secid>
  proton sequence
  proton sequence new --name=<seq-name> --audio=<audio-file> --duration=<duration> --frame-length=<frame-len> --layout-name=<layout-name>
  proton sequence get <seqid>
  proton sequence delete <seqid>
  proton sequence set-layout --seq=<seqid> --layout-name=<layout-name>
  proton permissions add Administrate --user=<user-name>
  proton permissions add EditSequence --user=<user-name> --sequence=<seqid>
  proton permissions add EditSection --user=<user-name> --sequence=<seqid> --section=<secid>
  proton permissions remove Administrate --user=<user-name>
  proton permissions remove EditSequence --user=<user-name> --sequence=<seqid>
  proton permissions remove EditSection --user=<user-name> --sequence=<seqid> --section=<secid>
  proton permissions list
  proton user add <user-name>
  proton user delete <user-name>
  proton user get <user-name>
  proton run update
  proton run repl
  proton run show
  proton vixen import-sequence --seq=<seq-file> --audio=<audio-file> --layout-name=<layout-name>
  proton vixen import-layout <layout-file>
  proton patch channel --proton=<proton-ch> --dmx=<dmx-ch>
  proton patch layout --patch=<patch-file> [--layout-name=<layout-name>]
Options:
  -h --help     Show this screen
";

// Docopt arguments are mapped to this struct
#[derive(Debug, RustcDecodable)]
pub struct DocoptArgs {
  pub arg_layout_file: Option<String>,
  pub arg_layout_name: Option<String>,
  pub arg_position: Option<u32>,
  pub arg_project_name: Option<String>,
  pub arg_secid: Option<u32>,
  pub arg_seqid: Option<u32>,
  pub arg_user_name: Option<String>,

  pub flag_audio: Option<String>,
  pub flag_data_file: Option<String>,
  pub flag_dmx: Option<u32>,
  pub flag_duration: Option<u32>,
  pub flag_end: Option<u32>,
  pub flag_fixtures: Option<Vec<u32>>,
  pub flag_frame_length: Option<u32>,
  pub flag_layout_file: Option<String>,
  pub flag_layout_name: Option<String>,
  pub flag_name: Option<String>,
  pub flag_patch: Option<String>,
  pub flag_position: Option<u32>,
  pub flag_proton_ch: Option<u32>,
  pub flag_section: Option<u32>,
  pub flag_sequence: Option<u32>,
  pub flag_seq: Option<String>,
  pub flag_start: Option<u32>,
  pub flag_user: Option<String>,
  
  pub cmd_add: Option<bool>,
  pub cmd_add_sequence: Option<bool>,
  pub cmd_administrate: Option<bool>,
  pub cmd_channel: Option<bool>,
  pub cmd_delete: Option<bool>,
  pub cmd_edit_sequence: Option<bool>,
  pub cmd_edit_section: Option<bool>,
  pub cmd_get: Option<bool>,
  pub cmd_import_layout: Option<bool>,
  pub cmd_import_sequence: Option<bool>,
  pub cmd_layout: Option<bool>,
  pub cmd_list: Option<bool>,
  pub cmd_new: Option<bool>,
  pub cmd_remove: Option<bool>,
  pub cmd_remove_sequence: Option<bool>,
  pub cmd_repl: Option<bool>,
  pub cmd_show: Option<bool>,
  pub cmd_set_layout: Option<bool>,
  pub cmd_update: Option<bool>,
}
