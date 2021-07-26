mod utils;

use std::process::exit;

use clap::{Arg, App, AppSettings};
use catnet::{participant::Participant, registrar::Registrar};
use log::{debug, info, error};

macro_rules! check_output {
    ($function:expr) => {
        match $function {
            Ok(i) => i,
            Err(e) => {
                error!("{}", e);

                exit(-1);
            }
        }
    };
}

fn main() {
    let matches =App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("Many authors, it's open source!")
        .about("A peer to peer, semi-decentralised, VPN.")
        .setting(AppSettings::SubcommandRequired)
        .arg(Arg::from("-l, --logging 'Sets a logging level to display.'")
             .required(false)
            .min_values(0)
            .default_value(&0.to_string())
        )
        .subcommand(App::new("registrar")
            .about("For those who want to start a cat.")
            .arg(Arg::from("<ip> -i, --ip 'The ip to host on.'").default_value("127.0.0.1").required(false))
            .arg(Arg::from("<port> -p, --port, 'The port to listen on.'").default_value("44400").required(false))
        )
        .subcommand(App::new("participant")
            .about("For those who want to join a cat.")
            .arg(Arg::from("<ip> -i, --ip 'The ip that the registrar exists on.'").default_value("127.0.0.1").required(false))
            .arg(Arg::from("<port> -p, --port 'The port that registrar exists on.'").default_value("44400").required(false))
        )
        .get_matches();

    utils::logging::init(matches.value_of("logging").unwrap().parse::<u64>().unwrap()).unwrap();

    debug!("matches:\n{:?}", matches);

    match matches.subcommand() {
        Some(("registrar", registrar_matches)) => {
            info!("Starting registrar....");
            let ip = registrar_matches.value_of("ip").expect("Expected a default value!");
            let port = registrar_matches.value_of("port").expect("Expected a default value!");
            let addr = ip.to_owned() + ":" + port;

            let mut registrar = check_output!(Registrar::from_addr(addr));
            check_output!(registrar.run());
        },

        Some(("participant", participant_matches)) => {
            info!("Starting participant....");
            let ip = participant_matches.value_of("ip").expect("Expected a default value!");
            let port = participant_matches.value_of("port").expect("Expected a defaut value!");
            let addr = ip.to_owned() + ":" + port;

            let mut participant = check_output!(Participant::from_addr(addr));
            check_output!(participant.run());
        }
        
        _ => unreachable!()
    }
}
