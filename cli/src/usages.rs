pub const USAGE: &'static str = "
operation -- A simple command line interface wallet for selendra/substrate.

Usage:
    operation <command> [<args>...]
    operation -h | --help
    operation --version
Commands:
    transfer           transfer token between account.
    balance            check amount of token.
    listaddresses      Prints the list of addresses and blance of each account.
    watchaddress       Add a watchonly address.
    getnewaddress      Generate a new address associated with label, deafult cryptography is sr25519
    restore            Restore address from json file
    backup             Backup specified address to local json file
Options:
    -h --help          Show this screen, or help about a command.
    -v --version       Show version.
See 'operation <command> --help' for information on a specific command.
";

pub const USAGE_TRANSACTION: &'static str = "
operation transfer -- transfer token between account.
Usage:
    operation transfer [-s <account>] [-p <password>] [-r <accountid>] [-a <amount>]
Options:
    -s --sender <account>       Your account account.
    -r --receiver <accountid>   Account Id you want to send.
    -a --amount <amount>        Amount of token to send
    -l  --location              Location of your wallet.
";

pub const USAGE_BALANCE: &'static str = "
operation balance -- check amount of token.
Usage:
    operation balance [-f <accountid>] [-t]
Options:
    -f --free-balance <accountid>   show free balance of account.
    -t --total-issuance             total amount of token in block chain.
";

pub const USAGE_LISTWALLET: &'static str = "
operation listaddresses -- Prints the list of addresses and blance of each account.
Usage:
    operation listaddresses [-l <location>]
Options:
    -l  --location    Location of your wallet.
";

pub const USAGE_WATCHADDRESS: &'static str = "
operation watchaddress -- Add a watchonly address.
Usage:
    operation watchaddress [-a <address>] [-n<name>]
Options:
    -a  --addr        Account address to save.
    -l  --location    Location of your wallet.
    -n  --name        Create account name default indracore.
";

pub const USAGE_RESTOREWALLET: &'static str = "
operation restore -- Restore address from json file.
Usage:
    operation restore [-f <diretory>] [-n<name>]
Options:
    -f  --file        File diretory or path.
    -l  --location    Location of your wallet.
    -p  --password    Password to decrypt file.
";

pub const USAGE_BACKUP: &'static str = "
operation backup  --  Backup specified address to local json file
Usage:
    operation backup [-f <diretory>] [-n<name>]
Options:
    -f  --file        File diretory or path.
    -l  --location    Location of your wallet.
    -a  --addr        Account address to save.
    -p  --password    Password to encrypt file.
";

pub const USAGE_GETWALLET: &'static str = "
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