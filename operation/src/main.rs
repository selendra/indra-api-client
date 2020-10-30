extern crate env_logger;
use std::env;

pub mod command;
use command::{api, operation, operation::Cmd};

use substrate_subxt::{ClientBuilder, DefaultNodeRuntime};

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().expect("Failed to read .env file");
    let url = std::env::var("RPC");

    let client = ClientBuilder::<DefaultNodeRuntime>::new()
        .set_url(url.unwrap())
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
        Ok(Cmd::Transaction(t)) => api::run_transaction(client, t).await,
        Err(msg) => {
            println!("{}", msg);
            std::process::exit(127);
        }
    };

    Ok(())
}
