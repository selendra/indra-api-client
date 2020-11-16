#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: String,
    pub location: Option<String>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ContractUpload {
    pub uploader: Option<String>,
    pub file: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Wallet {
    pub label: String,
    pub password: Option<String>,
    pub name: Option<String>,
    pub location: Option<String>,
    pub phrase: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ListWallet {
    pub location: Option<String>,
}

const USAGE: &'static str = "
operation -- operation infomation and transaction.

Usage:
    operation <command> [<args>...]
    operation -h | --help
    operation --version
Commands:
    transaction        transaction token between account.
    balance            check amount of token
Options:
    -h --help          Show this screen, or help about a command.
    -v --version       Show version.
See 'operation <command> --help' for information on a specific command.
";

const USAGE_TRANSACTION: &'static str = "
operation transaction -- transaction token between account.
Usage:
    operation transaction [-s <account>] [-p <password>] [-r <accountid>] [-a <amount>]
Options:
    -s --sender <account>       Your account account.
    -r --receiver <accountid>   Account Id you want to send.
    -a --amount <amount>        Amount of token to send
    -l  --location              Location of your wallet.
";

const USAGE_BALANCE: &'static str = "
operation balance -- check amount of token.
Usage:
    operation balance [-f <accountid>] [-t]
Options:
    -f --free-balance <accountid>   show free balance of account.
    -t --total-issuance             total amount of token in block chain.
";

const USAGE_LISTWALLET: &'static str = "
operation listaddresses -- Prints the list of addresses.
Usage:
    operation listaddresses [-l <location>]
Options:
    -l  --location   File location to save.
    -n  --name       Create account name default indracore.
";

const USAGE_GETWALLET: &'static str = "
operation getnewaddress -- A simple Command Line Interface wallet for Indracore.
Usage:
    operation getnewaddress [-s] [-n<AccountName>] [-p<Account>] [-a<//hard/soft///password>]
Options:
    -e, --ed25519   'Use Ed25519/BIP39 cryptography'
    -k, --ecdsa     'Use SECP256k1/ECDSA/BIP39 cryptography'
    -s, --sr25519   'Use Schnorr/Ristretto x25519/BIP39 cryptography'
    
    -n  --name       Create account name default indracore.
    -p  --password   Password //hard/soft///scret.
    -l  --location   File location to save.
        --phrase     Create account from phrase

";

// const USAGE_CONTRACT_UPLOAD: &'static str = "
// operation contract-upload -- upload smart contract.
// Usage:
//     operation contract-upload [-u <account>] [-f <file>]
// Options:
//     -u --uloader <account>   Your account account.
//     -f --file <file>          Your contract wasm file.
// ";

#[derive(Debug, Eq, PartialEq)]
pub enum Cmd {
    Transaction(Transaction),
    Balance(String),
    GetWallet(Wallet),
    ListWallet(ListWallet),
    // ContractUpload(ContractUpload),
    Version,
    Help(String),
}

pub fn print_usage(cmd: String) {
    match &cmd[..] {
        "operation" => println!("{}", &USAGE[1..]),
        "transaction" => println!("{}", &USAGE_TRANSACTION[1..]),
        "balance" => println!("{}", &USAGE_BALANCE[1..]),
        "getnewaddress" => println!("{}", &USAGE_GETWALLET[1..]),
        "listaddresses" => println!("{}", &USAGE_LISTWALLET[1..]),
        // "contract-upload" => println!("{}", &USAGE_CONTRACT_UPLOAD[1..]),
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
        Arg::Plain("transaction") => parse_transaction(args),
        Arg::Plain("balance") => parse_balance(args),
        Arg::Plain("getnewaddress") => parse_get_wallet(args),
        Arg::Plain("listaddresses") => parse_list_wallet(args),
        // Arg::Plain("contract-upload") => parse_contract_upload(args),
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
        Ok(Cmd::Balance(accountid.unwrap()))
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
                let msg = "Expected wallet name after --name.";
                name = Some(expect_plain(&mut args, msg)?);
            }
            Arg::Short("l") | Arg::Long("location") => {
                let msg = "Expected path or directoty after --location.";
                location = Some(expect_plain(&mut args, msg)?);
            }
            Arg::Long("phrase") => {
                let msg = "Expected path or directoty after --phrase.";
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
// fn parse_contract_upload(mut args: ArgIter) -> Result<Cmd, String> {
//     let mut uploader: Option<String> = None;
//     let mut file: Option<String> = None;

//     while let Some(arg) = args.next() {
//         match arg.as_ref() {
//             Arg::Short("u") | Arg::Long("uploader") => {
//                 let msg = "Expected account after --uploader.";
//                 uploader = Some(expect_plain(&mut args, msg)?);
//             }
//             Arg::Short("f") | Arg::Long("file") => {
//                 let msg = "Expected file after --file.";
//                 file = Some(expect_plain(&mut args, msg)?);
//             }
//             Arg::Short("h") | Arg::Long("help") => return drain_help(args, "contract-upload"),
//             _ => return unexpected(arg),
//         }
//     }
//     let upload = ContractUpload {
//         uploader: uploader,
//         file: file,
//     };
//     Ok(Cmd::ContractUpload(upload))
// }

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
            Arg::Short("h") | Arg::Long("help") => return drain_help(args, "transaction"),
            _ => return unexpected(arg),
        }
    }

    let transaction = Transaction {
        sender,
        receiver,
        amount,
        location,
    };

    Ok(Cmd::Transaction(transaction))
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
