extern crate env_logger;
use std::env;

pub mod command;
use command::{api, operation, operation::Cmd};

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let args = env::args().collect();
    match operation::parse(args) {
        Ok(Cmd::Help(cmd)) => operation::print_usage(cmd),
        Ok(Cmd::Version) => operation::print_version(),
        Ok(Cmd::Balance(cmd)) => api::check_balance(cmd).await,
        Ok(Cmd::ContractUpload(_cp)) => (),
        Ok(Cmd::Transaction(t)) => api::run_transaction(t).await,
        Err(msg) => {
            println!("{}", msg);
            std::process::exit(127);
        }
    };

    Ok(())
}
