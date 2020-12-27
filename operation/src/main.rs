extern crate env_logger;
use std::env;

use cli::{operation, operation::Cmd, transaction, wallet};

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let args = env::args().collect();
    match operation::parse(args) {
        Ok(Cmd::Help(cmd)) => operation::print_usage(cmd),
        Ok(Cmd::Version) => operation::print_version(),
        Ok(Cmd::Balance(cmd)) => transaction::check_balance(cmd).await,
        Ok(Cmd::GetWallet(wallet)) => wallet::get_wallet(wallet),
        Ok(Cmd::ListWallet(ls)) => wallet::list_wallet(ls),
        Ok(Cmd::WatchOnly(wl)) => wallet::watch_wallet(wl),
        Ok(Cmd::Restore(rw)) => wallet::restore_wallet(rw),
        Ok(Cmd::Backup(bp)) => wallet::backup(bp),
        Ok(Cmd::Transaction(tx)) => transaction::run_transaction(tx),
        Err(msg) => {
            println!("{}", msg);
            std::process::exit(127);
        }
    };

    Ok(())
}
