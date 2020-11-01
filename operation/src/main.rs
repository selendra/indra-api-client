extern crate env_logger;
use std::env;

pub mod command;
use command::{api, operation, operation::Cmd};
use utils::primitives::Config;

use substrate_subxt::{ClientBuilder, DefaultNodeRuntime};

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let client = ClientBuilder::<DefaultNodeRuntime>::new()
        .set_url(Config::url())
        .build()
        .await;

    let client = match client {
        Ok(c) => c,
        Err(_) => {
            colour::e_red_ln!("Connection refused");
            std::process::exit(111)
        }
    };

    let args = env::args().collect();
    match operation::parse(args) {
        Ok(Cmd::Help(cmd)) => operation::print_usage(cmd),
        Ok(Cmd::Version) => operation::print_version(),
        Ok(Cmd::Balance(cmd)) => api::check_balance(client, cmd).await,
        Ok(Cmd::ContractUpload(cp)) => api::contract(client, cp).await,
        Ok(Cmd::Transaction(t)) => api::run_transaction(client, t).await,
        Err(msg) => {
            println!("{}", msg);
            std::process::exit(127);
        }
    };

    Ok(())
}
