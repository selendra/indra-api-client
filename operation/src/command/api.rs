use colour::{dark_green_ln, e_red_ln, e_yellow_ln};
use substrate_subxt::{
    balances::*, contracts::*, sp_core::Decode, system::*, Error, EventSubscription, EventsDecoder,
    ExtrinsicSuccess, IndracoreNodeRuntime,
};
use utils::{
    keyring,
    primitives::{Client, Config, ContractUpload, Signer, Token, Transaction},
    read,
};

pub async fn run_transaction(client: Client, transaction: Transaction) {
    let amount = Token::get(transaction.amount.unwrap());
    let sender = keyring::Signer::new(transaction.sender.unwrap());
    let reciever = keyring::AccountId::new(transaction.receiver.unwrap());

    let info = client.account(&sender.accountid(), None).await;
    let info = match info {
        Ok(info) => info,
        Err(_) => {
            e_red_ln!("!!! something went wrong");
            std::process::exit(1)
        }
    };

    if info.data.free <= amount.pay() {
        let amount = Token::amount(info.data.free);
        e_yellow_ln!(
            "!!! your balance : {} {} too low to send",
            amount,
            Config::token()
        )
    } else {
        let sub = client.subscribe_events().await;
        let sub = match sub {
            Ok(sub) => sub,
            Err(_) => {
                e_red_ln!("!!! something went wrong");
                std::process::exit(1)
            }
        };
        let mut decoder = EventsDecoder::<IndracoreNodeRuntime>::new(client.metadata().clone());
        decoder.with_balances();
        let mut sub = EventSubscription::<IndracoreNodeRuntime>::new(sub, decoder);
        sub.filter_event::<TransferEvent<_>>();
        let hash = client
            .transfer(&sender.pair(), &reciever.accounid(), amount.pay())
            .await;
        let hash = match hash {
            Ok(h) => h,
            Err(_) => {
                e_red_ln!("!!! Transaction failed");
                std::process::exit(1)
            }
        };
        let raw = sub.next().await.unwrap().unwrap();

        let event = TransferEvent::<IndracoreNodeRuntime>::decode(&mut &raw.data[..]);
        if let Ok(event) = event {
            colour::dark_cyan_ln!(
                ">> Balance transfer extrinsic submitted: {}\n\t** from: {:?}\n\t** to: {:?}\n\t** amount {} {}",
                hash, event.from, event.to, amount.token, Config::token()
            );
        } else {
            e_red_ln!("!!! Failed to subscribe to Balances::Transfer Event");
        }
    }
}

pub async fn check_balance(client: Client, cmd: String) {
    if cmd.eq("total-issuance") {
        let total = client.total_issuance(None).await.unwrap();
        let amount = Token::lamount(total);
        dark_green_ln!("***total issuance is: {:?} {}", amount, Config::token())
    } else {
        let account = keyring::AccountId::new(cmd);
        let info = client.account(&account.accounid32(), None).await;
        let info = match info {
            Ok(info) => info,
            Err(_) => {
                e_red_ln!("!!! something went wrong");
                std::process::exit(1)
            }
        };

        let amount = Token::amount(info.data.free);
        dark_green_ln!("*** your free balance is {} {}", amount, Config::token())
    }
}

pub async fn contract(client: Client, contract: ContractUpload) {
    let uploader = keyring::Signer::new(contract.uploader.unwrap());

    let code_stored = put_code(client.clone(), uploader.pair(), contract.file.unwrap()).await;
    let code_stored = match code_stored {
        Ok(cs) => cs,
        Err(e) => {
            e_red_ln!("!!! Contract Upload failed {:?}", e);
            std::process::exit(1)
        }
    };
    let instant = deploy(client.clone(), uploader.pair(), &code_stored.code_hash, &[]).await;
    let instant = match instant {
        Ok(cs) => cs,
        Err(e) => {
            e_red_ln!("!!! Contract Deploy failed {:?}", e);
            std::process::exit(1)
        }
    };
    dark_green_ln!("*** contract code hash: {:?}", code_stored.code_hash);
    dark_green_ln!(
        "*** contract instantiate hash: {}",
        instant.contract.clone()
    );
    let id = instant.contract.clone();
    let contract = pallet_indices::address::Address::from(id);
    let call = call_contract(client, &contract, &[], uploader.pair()).await;
    println!("{:?}", call);
}

pub async fn put_code(
    client: Client,
    uploader: Signer,
    file: String,
) -> Result<CodeStoredEvent<IndracoreNodeRuntime>, Error> {
    let w = read::read_wasm(file);
    let result = client.put_code_and_watch(&uploader, &w).await?;

    let code_stored = result
        .code_stored()?
        .ok_or_else(|| Error::Other("Failed to find a CodeStored event".into()))?;
    Ok(code_stored)
}

pub async fn deploy(
    client: Client,
    uploader: Signer,
    code_hash: &<IndracoreNodeRuntime as System>::Hash,
    data: &[u8],
) -> Result<InstantiatedEvent<IndracoreNodeRuntime>, Error> {
    let result = client
        .instantiate_and_watch(
            &uploader,
            1_000_000_000_000_000,
            500_000_000,
            code_hash,
            data,
        )
        .await?;

    let instantiated = result
        .instantiated()?
        .ok_or_else(|| Error::Other("Failed to find a Instantiated event".into()))?;

    Ok(instantiated)
}

async fn call_contract(
    client: Client,
    contract: &<IndracoreNodeRuntime as System>::Address,
    input_data: &[u8],
    uploader: Signer,
) -> Result<ExtrinsicSuccess<IndracoreNodeRuntime>, Error> {
    let result = client
        .call_and_watch(
            &uploader,
            contract,
            0,           // value
            500_000_000, // gas_limit
            input_data,
        )
        .await?;
    Ok(result)
}
