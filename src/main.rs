#[macro_use]
extern crate clap;
extern crate env_logger;
extern crate keystore_lib;

use clap::App;

fn main() {
    env_logger::init();

    let yaml = load_yaml!("cli.yml");
    let mut app = App::from_yaml(yaml);
    let matches = app.clone().get_matches();

    if let Some(_) = matches.subcommand_matches("config") {
        keystore_lib::print_config();
    } else if let Some(_) = matches.subcommand_matches("server") {
        keystore_lib::start_server();
    } else {
        let _ = app.print_help();
        println!("\n")
    }
}