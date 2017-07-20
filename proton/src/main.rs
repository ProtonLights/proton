extern crate docopt;
extern crate protonlib;

use std::env;
use protonlib::DocoptArgs;
use protonlib::handlers;
use protonlib::ProtonConfig;
use docopt::Docopt;

fn main() {

  // Get command line arguments using Docopt. Provides guarantees about input types.
  let args: DocoptArgs = Docopt::new(protonlib::USAGE)
    .and_then(|d| d.deserialize())
    .unwrap_or_else(|e| e.exit());

  // Get first argument (tells us what to do)
  let command = env::args().nth(1).unwrap();

  // Check if initial setup (avoid loading config file if not created yet)
  if command == "setup" {
    protonlib::setup::initial_setup(args);
    return;
  }
  
  // Load configuration file
  let config = ProtonConfig::load().expect("Failed to load config file");

  if command == "configure" {
    protonlib::setup::configure(config);
    return;
  }
  
  // Handle command based on first argument
  let handler: fn(DocoptArgs, ProtonConfig) = match command.as_ref() {
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
