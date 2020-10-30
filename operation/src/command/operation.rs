use utils::primitives::Transaction;

const USAGE: &'static str = "
operation -- operation infomation and transaction.

Usage:
    operation <command> [<args>...]
    operation -h | --help
    operation --version
Commands:
    transaction        transaction token between account.
    free-operation       your total account token.
    total-issuance     total token in indracore.
Options:
    -h --help          Show this screen, or help about a command.
    -v --version       Show version.
See 'operation <command> --help' for information on a specific command.
";

const USAGE_TRANSACTION: &'static str = "
operation transaction -- transaction token between account.
Usage:
    operation transaction [-s <mnemonic>] [-r <accountid>] [-a <amount>]
Options:
    -s --sender <mnemonic>     Your account mnemonic.
    -r --receiver <accountid>  Account Id you want to send.
    -a --amount <amount>       Amount of token to send
";

#[derive(Debug, Eq, PartialEq)]
pub enum Cmd {
    Transaction(Transaction),
    // Freeoperation,
    // TotalIssuance,
    Version,
    Help(String),
}

pub fn print_usage(cmd: String) {
    match &cmd[..] {
        "operation" => println!("{}", &USAGE[1..]),
        "transaction" => println!("{}", &USAGE_TRANSACTION[1..]),
        _ => println!("'{}' is not a Tako command. See 'operation --help'.", cmd),
    }
}

pub fn print_version() {
    println!(
        "
    - version 0.1.0
    - indracore operation with command line
    "
    );
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
        Arg::Long("version") => drain(args).and(Ok(Cmd::Version)),
        Arg::Short("h") | Arg::Long("help") => parse_help(args),
        _ => return unexpected(arg),
    }
}

fn parse_transaction(mut args: ArgIter) -> Result<Cmd, String> {
    let mut sender: Option<String> = None;
    let mut receiver: Option<String> = None;
    let mut amount: Option<String> = None;

    while let Some(arg) = args.next() {
        match arg.as_ref() {
            Arg::Short("s") | Arg::Long("sender") => {
                let msg = "Expected mnemonic after --sender.";
                sender = Some(expect_plain(&mut args, msg)?);
            }
            Arg::Short("r") | Arg::Long("receiver") => {
                let msg = "Expected account id after --receiver.";
                receiver = Some(expect_plain(&mut args, msg)?);
            }
            Arg::Short("a") | Arg::Long("amount") => {
                let msg = "Expected amount of token after --amount.";
                amount = Some(expect_plain(&mut args, msg)?);
            }
            Arg::Short("h") | Arg::Long("help") => return drain_help(args, "transaction"),
            _ => return unexpected(arg),
        }
    }

    let transaction = Transaction {
        sender: sender,
        receiver: receiver,
        amount: amount,
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
