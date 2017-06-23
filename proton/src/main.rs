extern crate docopt;
extern crate protonlib;

use std::env;
use protonlib::DocoptArgs;
use protonlib::handlers;
use protonlib::ProtonConfig;
use docopt::Docopt;

fn main() {
  // Load configuration file
  let config = ProtonConfig::load("proton.cfg".to_string());

  // Get command line arguments using Docopt. Provides guarantees about input types.
  let args: DocoptArgs = Docopt::new(protonlib::USAGE)
    .and_then(|d| d.decode())
    .unwrap_or_else(|e| e.exit());

  // Handle command based on first argument
  let handler: fn(DocoptArgs, ProtonConfig) = match env::args().nth(1).unwrap().as_ref() {
    "layout" => handlers::handle_layout,
    "patch" => handlers::handle_patch,
    "permissions" => handlers::handle_permissions,
    "project" => handlers::handle_project,
    "run" => handlers::handle_run,
    "section" => handlers::handle_section,
    "sequence" => handlers::handle_sequence,
    "user" => handlers::handle_user,
    "vixen" => handlers::handle_vixen,
    _ => panic!("Invalid first argument"),
  };

  handler(args, config);
}
