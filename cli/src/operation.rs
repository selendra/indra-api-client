pub use crate::models::{Transaction, Wallet, WatchWallet, RestoreWallet, ListWallet, Backup};
use crate::usages::{
    USAGE, USAGE_TRANSACTION, USAGE_BALANCE, USAGE_LISTWALLET,
    USAGE_WATCHADDRESS, USAGE_RESTOREWALLET, USAGE_BACKUP, USAGE_GETWALLET
};

#[derive(Debug, Eq, PartialEq)]
pub enum Cmd {
    Transaction(Transaction),
    Balance(String),
    GetWallet(Wallet),
    ListWallet(ListWallet),
    WatchOnly(WatchWallet),
    Restore(RestoreWallet),
    Backup(Backup),
    Version,
    Help(String),
}

pub fn print_usage(cmd: String) {
    match &cmd[..] {
        "operation" => println!("{}", &USAGE[1..]),
        "transfer" => println!("{}", &USAGE_TRANSACTION[1..]),
        "balance" => println!("{}", &USAGE_BALANCE[1..]),
        "getnewaddress" => println!("{}", &USAGE_GETWALLET[1..]),
        "listaddresses" => println!("{}", &USAGE_LISTWALLET[1..]),
        "watchaddress" => println!("{}", &USAGE_WATCHADDRESS[1..]),
        "restore" => println!("{}", &USAGE_RESTOREWALLET[1..]),
        "backup" => println!("{}", &USAGE_BACKUP[1..]),
        _ => println!(
            "'{}' is not a Operation command. See 'operation --help'.",
            cmd
        ),
    }
}

pub fn print_version() {
    println!("operation version 0.1.0");
}

enum Arg<T> {
    Plain(T),
    Short(T),
    Long(T),
}

impl Arg<String> {
    fn as_ref(&self) -> Arg<&str> {
        match *self {
            Arg::Plain(ref x) => Arg::Plain(&x[..]),
            Arg::Short(ref x) => Arg::Short(&x[..]),
            Arg::Long(ref x) => Arg::Long(&x[..]),
        }
    }

    #[allow(dead_code)]
    fn into_string(self) -> String {
        match self {
            Arg::Plain(x) => x,
            Arg::Short(x) => x,
            Arg::Long(x) => x,
        }
    }
}

impl std::fmt::Display for Arg<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Arg::Plain(ref x) => write!(f, "{}", x),
            Arg::Short(ref x) => write!(f, "-{}", x),
            Arg::Long(ref x) => write!(f, "--{}", x),
        }
    }
}

struct ArgIter {
    args: std::vec::IntoIter<String>,
    is_raw: bool,
    leftover: Option<String>,
}

impl ArgIter {
    pub fn new(args: Vec<String>) -> ArgIter {
        ArgIter {
            args: args.into_iter(),
            is_raw: false,
            leftover: None,
        }
    }
}

impl Iterator for ArgIter {
    type Item = Arg<String>;

    fn next(&mut self) -> Option<Arg<String>> {
        if self.leftover.is_some() {
            return self.leftover.take().map(Arg::Plain);
        }

        let arg = self.args.next()?;

        if self.is_raw {
            return Some(Arg::Plain(arg));
        }

        if &arg == "--" {
            self.is_raw = true;
            return self.next();
        }

        if arg.starts_with("--") {
            let mut flag = String::from(&arg[2..]);
            if let Some(i) = flag.find('=') {
                self.leftover = Some(flag.split_off(i + 1));
                flag.truncate(i);
            }
            return Some(Arg::Long(flag));
        }

        if arg.starts_with("-") {
            let mut flag = String::from(&arg[1..]);
            if flag.len() > 1 {
                self.leftover = Some(flag.split_off(1));
                flag.truncate(1);
            }
            return Some(Arg::Short(flag));
        }

        Some(Arg::Plain(arg))
    }
}

pub fn parse(argv: Vec<String>) -> Result<Cmd, String> {
    let mut args = ArgIter::new(argv);

    // Skip executable name.
    args.next();

    let arg = match args.next() {
        Some(a) => a,
        None => return Err("No command provided. See --help.".to_string()),
    };

    match arg.as_ref() {
        Arg::Plain("transfer") => parse_transaction(args),
        Arg::Plain("balance") => parse_balance(args),
        Arg::Plain("getnewaddress") => parse_get_wallet(args),
        Arg::Plain("listaddresses") => parse_list_wallet(args),
        Arg::Plain("watchaddress") => parse_watchonly(args),
        Arg::Plain("restore") => parse_restore(args),
        Arg::Plain("backup") => parse_backup(args),
        Arg::Long("version") => drain(args).and(Ok(Cmd::Version)),
        Arg::Short("h") | Arg::Long("help") => parse_help(args),
        _ => return unexpected(arg),
    }
}

fn parse_balance(mut args: ArgIter) -> Result<Cmd, String> {
    let mut total_issuance: bool = false;
    let mut accountid: Option<String> = None;

    while let Some(arg) = args.next() {
        match arg.as_ref() {
            Arg::Short("t") | Arg::Long("total-issuance") => total_issuance = true,
            Arg::Short("f") | Arg::Long("free-balance") => {
                let msg = "Expected account id after --free-balance.";
                accountid = Some(expect_plain(&mut args, msg)?);
            }
            Arg::Short("h") | Arg::Long("help") => return drain_help(args, "balance"),
            _ => return unexpected(arg),
        }
    }
    if total_issuance {
        Ok(Cmd::Balance("total-issuance".to_string()))
    } else {
        let accountid = match accountid {
            Some(id) => id,
            None => "".to_owned()
        };
        Ok(Cmd::Balance(accountid))
    }
}

fn parse_list_wallet(mut args: ArgIter) -> Result<Cmd, String> {
    let mut location: Option<String> = None;
    while let Some(arg) = args.next() {
        match arg.as_ref() {
            Arg::Short("l") | Arg::Long("location") => {
                let msg = "Expected path or directoty after --location.";
                location = Some(expect_plain(&mut args, msg)?);
            }

            Arg::Short("h") | Arg::Long("help") => return drain_help(args, "listaddresses"),
            _ => return unexpected(arg),
        }
    }
    Ok(Cmd::ListWallet(ListWallet { location }))
}

fn parse_watchonly(mut args: ArgIter) -> Result<Cmd, String> {
    let mut location: Option<String> = None;
    let mut name: Option<String> = None;
    let mut address: Option<String> = None;
    while let Some(arg) = args.next() {
        match arg.as_ref() {
            Arg::Short("a") | Arg::Long("addr") => {
                let msg = "Expected account address after --location.";
                address = Some(expect_plain(&mut args, msg)?);
            }
            Arg::Short("l") | Arg::Long("location") => {
                let msg = "Expected path or directoty after --location.";
                location = Some(expect_plain(&mut args, msg)?);
            }
            Arg::Short("n") | Arg::Long("name") => {
                let msg = "Expected account name after --name.";
                name = Some(expect_plain(&mut args, msg)?);
            }

            Arg::Short("h") | Arg::Long("help") => return drain_help(args, "watchaddress"),
            _ => return unexpected(arg),
        }
    }
    Ok(Cmd::WatchOnly(WatchWallet {
        location,
        name,
        address,
    }))
}

fn parse_restore(mut args: ArgIter) -> Result<Cmd, String> {
    let mut location: Option<String> = None;
    let mut file: Option<String> = None;
    let mut password: Option<String> = None;
    while let Some(arg) = args.next() {
        match arg.as_ref() {
            Arg::Short("f") | Arg::Long("file") => {
                let msg = "Expected file diretory or path after --location.";
                file = Some(expect_plain(&mut args, msg)?);
            }
            Arg::Short("l") | Arg::Long("location") => {
                let msg = "Expected path or directoty after --location.";
                location = Some(expect_plain(&mut args, msg)?);
            }
            Arg::Short("p") | Arg::Long("password") => {
                let msg = "Expected password after --password.";
                password = Some(expect_plain(&mut args, msg)?);
            }

            Arg::Short("h") | Arg::Long("help") => return drain_help(args, "restore"),
            _ => return unexpected(arg),
        }
    }
    Ok(Cmd::Restore(RestoreWallet {
        location,
        file,
        password,
    }))
}

fn parse_backup(mut args: ArgIter) -> Result<Cmd, String> {
    let mut location: Option<String> = None;
    let mut address: Option<String> = None;
    let mut file: Option<String> = None;
    let mut password: Option<String> = None;
    while let Some(arg) = args.next() {
        match arg.as_ref() {
            Arg::Short("f") | Arg::Long("file") => {
                let msg = "Expected file diretory or path after --location.";
                file = Some(expect_plain(&mut args, msg)?);
            }
            Arg::Short("l") | Arg::Long("location") => {
                let msg = "Expected path or directoty after --location.";
                location = Some(expect_plain(&mut args, msg)?);
            }
            Arg::Short("a") | Arg::Long("addr") => {
                let msg = "Expected account address after --name.";
                address = Some(expect_plain(&mut args, msg)?);
            }
            Arg::Short("p") | Arg::Long("password") => {
                let msg = "Expected password after --password.";
                password = Some(expect_plain(&mut args, msg)?);
            }

            Arg::Short("h") | Arg::Long("help") => return drain_help(args, "backup"),
            _ => return unexpected(arg),
        }
    }
    Ok(Cmd::Backup(Backup {
        location,
        address,
        file,
        password,
    }))
}

fn parse_get_wallet(mut args: ArgIter) -> Result<Cmd, String> {
    let mut ed25519: bool = false;
    let mut ecdsa: bool = false;
    let mut sr25519: bool = false;

    let mut password: Option<String> = None;
    let mut name: Option<String> = None;
    let mut location: Option<String> = None;
    let mut phrase: Option<String> = None;

    while let Some(arg) = args.next() {
        match arg.as_ref() {
            Arg::Short("e") | Arg::Long("ed25519") => ed25519 = true,
            Arg::Short("k") | Arg::Long("ecdsa") => ecdsa = true,
            Arg::Short("s") | Arg::Long("sr25519") => sr25519 = true,

            Arg::Short("p") | Arg::Long("password") => {
                let msg = "Expected password after --password.";
                password = Some(expect_plain(&mut args, msg)?);
            }
            Arg::Short("n") | Arg::Long("name") => {
                let msg = "Expected account name after --name.";
                name = Some(expect_plain(&mut args, msg)?);
            }
            Arg::Short("l") | Arg::Long("location") => {
                let msg = "Expected path or directoty after --location.";
                location = Some(expect_plain(&mut args, msg)?);
            }
            Arg::Long("phrase") => {
                let msg = "Expected mnemonic or seed after --phrase.";
                phrase = Some(expect_plain(&mut args, msg)?);
            }

            Arg::Short("h") | Arg::Long("help") => return drain_help(args, "getnewaddress"),
            _ => return unexpected(arg),
        }
    }
    let label: &str;

    if ed25519 {
        label = "ed25519";
    } else if ecdsa {
        label = "ed25519";
    } else if sr25519 {
        label = "sr25519";
    } else {
        label = "sr25519";
    };

    Ok(Cmd::GetWallet(Wallet {
        label: label.to_string(),
        password,
        name,
        location,
        phrase,
    }))
}

fn parse_transaction(mut args: ArgIter) -> Result<Cmd, String> {
    let mut sender: String = "".into();
    let mut receiver: String = "".into();
    let mut amount: String = "".into();
    let mut location: Option<String> = None;

    while let Some(arg) = args.next() {
        match arg.as_ref() {
            Arg::Short("s") | Arg::Long("sender") => {
                let msg = "Expected account after --sender.";
                sender = expect_plain(&mut args, msg)?;
            }
            Arg::Short("r") | Arg::Long("receiver") => {
                let msg = "Expected account id after --receiver.";
                receiver = expect_plain(&mut args, msg)?;
            }
            Arg::Short("a") | Arg::Long("amount") => {
                let msg = "Expected amount of token after --amount.";
                amount = expect_plain(&mut args, msg)?;
            }
            Arg::Short("l") | Arg::Long("location") => {
                let msg = "Expected path or directoty after --location.";
                location = Some(expect_plain(&mut args, msg)?);
            }
            Arg::Short("h") | Arg::Long("help") => return drain_help(args, "transfer"),
            _ => return unexpected(arg),
        }
    }

    let transfer = Transaction {
        sender,
        receiver,
        amount,
        location,
    };

    Ok(Cmd::Transaction(transfer))
}

fn parse_help(mut args: ArgIter) -> Result<Cmd, String> {
    match args.next() {
        Some(Arg::Plain(cmd)) => drain(args).and(Ok(Cmd::Help(cmd))),
        Some(arg) => unexpected(arg),
        None => Ok(Cmd::Help("operation".to_string())),
    }
}

fn drain_help(args: ArgIter, cmd: &'static str) -> Result<Cmd, String> {
    drain(args).and(Ok(Cmd::Help(cmd.to_string())))
}

fn expect_plain(args: &mut ArgIter, msg: &'static str) -> Result<String, String> {
    match args.next() {
        Some(Arg::Plain(a)) => Ok(a),
        Some(arg) => Err(format!("Unexpected argument '{}'. {}", arg, msg)),
        None => Err(msg.to_string()),
    }
}

fn drain(args: ArgIter) -> Result<(), String> {
    for arg in args {
        return unexpected::<()>(arg);
    }

    Ok(())
}

fn unexpected<T>(arg: Arg<String>) -> Result<T, String> {
    Err(format!(
        "Unexpected argument '{}'. See 'operation --help'.",
        arg
    ))
}
