mod utils;

use clap::{Arg, App};
use log::*;

fn main() {
    let matches =App::new("catnetd")
        .version("1.0.0")
        .author("Many authors, it's open source!")
        .about("A peer to peer, semi-decentralised, VPN.")
        .arg(Arg::from("-l, --logging 'Sets a logging level to display'")
             .required(false)
            .min_values(0)
            .default_value(&0.to_string())
        )
        .subcommand(App::new("registrar")
            .about("For those who want to start a cat"))
        .subcommand(App::new("participant")
            .about("For those who want to join a cat"))
        .get_matches();

    utils::logging::init(matches.value_of("logging").unwrap().parse::<u64>().unwrap());

    trace!("trace");
    debug!("debug");
    info!("info");
    warn!("warn");
    error!("error");
}
