// GAF cli tool
// GAF is a package manager for any language built using rust

mod gaf;

use std::env;
use std::process;

/*
use gaf::cli::Cli;
use gaf::commands::Commands;
use gaf::config::Config;
use gaf::install::Install;
use gaf::gaf::Gaf;
*/

fn main(ARGS: Vec<String>) {
    // Check to see if the arguments include help

    if ARGS.contains(&String::from("--help")) {
        println!("GAF is a package manager for any language built using rust");
        println!("Usage: gaf [command] [options]");
        println!("Commands:");
        println!("  install [package] [options] - Install a package");
        println!("  uninstall [package] [options] - Uninstall a package");
        println!("  update [package] [options] - Update a package");
        println!("  search [package] [options] - Search for a package");
        println!("  list [package] [options] - List all installed packages");
        println!("  config [package] [options] - Configure a package");
        println!("  help [package] [options] - Display help");
        println!("  version [package] [options] - Display version");
        println!("  info [package] [options] - Display info about a package");
        println!("  init [package] [options] - Initialize a package");
        println!("  publish [package] [options] - Publish a package");
        println!("  unpublish [package] [options] - Unpublish a package");
        println!("  login [package] [options] - Login to a package manager");
        println!("  logout [package] [options] - Logout of a package manager");
        println!("  whoami [package] [options] - Display who you are logged in as");
        println!("  adduser [package] [options] - Add a user to a package manager");
        println!("  removeuser [package] [options] - Remove a user from a package manager");
        println!("  owner [package] [options] - Display the owner of a package");
        println!("  deprecate [package] [options] - Deprecate a package");
        println!("  undeprecate [package] [options] - Undeprecate a package");
        println!("  star [package] [options] - Star a package");
        println!("  unstar [package] [options] - Unstar a package");
        println!("  stars [package] [options] - Display all stars");
        println!("  tag [package] [options] - Tag a package");
        println!("  untag [package] [options] - Untag a package");
        println!("  tags [package] [options] - Display all tags");
        println!("  test [package] [options] - Test a package");
        println!("  run [package] [options] - Run a package");
        println!("  build [package] [options] - Build a package");
    } else {
        println!("Invalid command. Use gaf --help for help");
    }

    process::exit(0);
}