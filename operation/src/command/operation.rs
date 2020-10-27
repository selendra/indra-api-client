use clap::{App, Arg, SubCommand};
use colour::{dark_green_ln, dark_grey_ln, e_yellow_ln};
use substrate_subxt::{balances::*, system::*};
use utils::{
    keyring,
    primitives::{Client, Token},
};

pub async fn operation_cmd(client: Client) {
    let matches = App::new("Indracore Operations")
        .version("1.0")
        .author("Selendra <info@selendra.org>")
        .about("operation of Indracore blockchain")
        .arg(
            Arg::with_name("total-issuance")
                .long("total-issuance")
                .help("total token that issuance"),
        )
        .arg(
            Arg::with_name("free-balance")
                .long("free-balance")
                .takes_value(true)
                .help("your free balance \n--free_balance `mnemonic` "),
        )
        .subcommand(
            SubCommand::with_name("transation")
                .about("operation transaction [mnemonic] [accountid")
                .version("1.0.0")
                .arg(
                    Arg::with_name("mnemonic")
                        .short("m")
                        .long("mnemonic")
                        .takes_value(true)
                        .help("your account mnemonic want use \n --mnemonic `mnemonic`"),
                )
                .arg(
                    Arg::with_name("accountid")
                        .short("a")
                        .long("accountid")
                        .takes_value(true)
                        .help("your account id want to send"),
                )
                .arg(
                    Arg::with_name("token")
                        .short("t")
                        .long("token")
                        .takes_value(true)
                        .help("token amount want to send"),
                ),
        )
        .get_matches();

    if matches.is_present("total-issuance") {
        let total_issuance = client.total_issuance(None).await.unwrap();
        dark_green_ln!("*** total issuance is :{}", total_issuance)
    }

    if matches.is_present("free-balance") {
        let mnemonic = matches.value_of("free-balance").unwrap_or("");
        let account = keyring::Signer::to_accountid(mnemonic);
        let info = client.account(&account, None).await.unwrap();
        dark_green_ln!("*** your free balance is :{}", info.data.free)
    }

    if let Some(matches) = matches.subcommand_matches("transation") {
        if matches.is_present("mnemonic")
            && matches.is_present("accountid")
            && matches.is_present("token")
        {
            let mnemonic = matches.value_of("mnemonic").unwrap_or("");
            let accountid = matches.value_of("accountid").unwrap_or("");
            let token = matches.value_of("token").unwrap_or("0.0");
            let amount = Token::amount(token.parse::<f64>().unwrap());

            let account = keyring::Signer::to_accountid(mnemonic);
            let info = client.account(&account, None).await.unwrap();

            if info.data.free <= amount {
                e_yellow_ln!("!!! your balance : {:?} too low to send", info.data.free)
            } else {
                let sender = keyring::Signer::from_mnemonic(mnemonic);
                let receiver = keyring::Signer::from_accountid(accountid);
                let hash = client.transfer(&sender, &receiver, amount).await;

                dark_green_ln!(
                    ">> Balance transfer extrinsic submitted: {:?}
                    * from {:?}
                    * to {:?}
                    * amount = {:?}
                    ",
                    hash,
                    account,
                    receiver,
                    token
                );
            }
        } else {
            dark_grey_ln!("--help for more info");
        }
    }
}
