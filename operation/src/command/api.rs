use substrate_subxt::{
    balances::*, sp_core::Decode, system::*, DefaultNodeRuntime, EventSubscription, EventsDecoder,
};
use utils::keyring;
use utils::primitives::{Client, Token, Transaction};

pub async fn run_transaction(client: Client, transaction: Transaction) {
    let amount = Token::get(transaction.amount.unwrap());
    let sender = keyring::Signer::new(transaction.sender.unwrap());
    let reciever = keyring::Receiver::new(transaction.receiver.unwrap());

    let info = client.account(&sender.accountid(), None).await.unwrap();

    if info.data.free <= amount.pay() {
        colour::e_yellow_ln!("!!! your balance : {:?} too low to send", info.data.free)
    } else {
        let sub = client.subscribe_events().await;
        let sub = match sub {
            Ok(sub) => sub,
            Err(_) => {
                colour::e_red_ln!("something went wrong");
                std::process::exit(1)
            }
        };
        let mut decoder = EventsDecoder::<DefaultNodeRuntime>::new(client.metadata().clone());
        decoder.with_balances();
        let mut sub = EventSubscription::<DefaultNodeRuntime>::new(sub, decoder);
        sub.filter_event::<TransferEvent<_>>();
        let hash = client
            .transfer(&sender.pair(), &reciever.accounid(), amount.pay())
            .await;
        let hash = match hash {
            Ok(h) => h,
            Err(_) => {
                colour::e_red_ln!("Transaction failed");
                std::process::exit(1)
            }
        };
        let raw = sub.next().await.unwrap().unwrap();

        let event = TransferEvent::<DefaultNodeRuntime>::decode(&mut &raw.data[..]);
        if let Ok(event) = event {
            colour::dark_cyan_ln!(
                "Balance transfer extrinsic submitted: {}\n\t** from: {:?}\n\t** to: {:?}\n\t** amount {} tsel",
                hash, event.from, event.to, amount.token
            );
        } else {
            println!("Failed to subscribe to Balances::Transfer Event");
        }
    }
}
