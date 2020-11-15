extern crate env_logger;
use std::env;

pub mod command;
use command::{operation, operation::Cmd, transaction, wallet};

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let args = env::args().collect();
    match operation::parse(args) {
        Ok(Cmd::Help(cmd)) => operation::print_usage(cmd),
        Ok(Cmd::Version) => operation::print_version(),
        Ok(Cmd::Balance(cmd)) => transaction::check_balance(cmd).await,
        // Ok(Cmd::ContractUpload(_cp)) => (),
        Ok(Cmd::GetWallet(wallet)) => wallet::get_wallet(wallet),
        Ok(Cmd::ListWallet(ls)) => wallet::list_wallet(ls),
        Ok(Cmd::Transaction(tx)) => transaction::run_transaction(tx).await,
        Err(msg) => {
            println!("{}", msg);
            std::process::exit(127);
        }
    };

    Ok(())
}
