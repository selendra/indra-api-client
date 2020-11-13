use substrate_subxt::{balances::*, sp_core, ClientBuilder, Error, IndracoreNodeRuntime};

use crate::{keyring::Sr25519, primitives};
pub struct Transaction {
    pub sender: Sr25519,
    pub reciever: primitives::IndracoreId,
    pub amount: u128,
}

impl Transaction {
    pub fn run(&self, pass: Option<&str>) -> Result<sp_core::H256, Error> {
        let sender = match self.sender.pair(pass) {
            Ok(pair) => pair,
            Err(e) => return Err(e),
        };

        async_std::task::block_on(async move {
            let client = match ClientBuilder::<IndracoreNodeRuntime>::new()
                .set_url(primitives::url())
                .build()
                .await
            {
                Ok(cli) => cli,
                Err(e) => return Err(e),
            };

            let hash = match client.transfer(&sender, &self.reciever, self.amount).await {
                Ok(hash) => hash,
                Err(e) => return Err(e),
            };
            Ok(hash)
        })
    }
}
