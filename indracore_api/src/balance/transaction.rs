use crate::primitives;
use substrate_subxt::{
    balances::*, sp_core, ClientBuilder, Error, IndracoreNodeRuntime,
};

pub struct Transaction {
    pub sender: primitives::Sr25519,
    pub reciever: primitives::IndracoreId,
    pub amount: u128,
}

impl Transaction {
    pub fn run(&self) -> Result<sp_core::H256, Error> {
        async_std::task::block_on(async move {
            let client = match ClientBuilder::<IndracoreNodeRuntime>::new()
                .set_url(primitives::url())
                .build()
                .await
            {
                Ok(cli) => cli,
                Err(e) => return Err(e),
            };

            let hash = match client.transfer(&self.sender, &self.reciever, self.amount).await {
                Ok(hash) => hash,
                Err(e) => return Err(e),
            };
            Ok(hash)
        })
    }
}
