use crate::primitives;
use sp_core::sr25519::Pair;
use substrate_subxt::{
    balances::*, sp_core, ClientBuilder, Error, IndracoreNodeRuntime, PairSigner,
};

pub struct Transaction {
    pub sender: Pair,
    pub reciever: primitives::IndracoreId,
    pub amount: u128,
}

impl Transaction {
    pub fn run(&self) -> Result<sp_core::H256, Error> {
        let signer = PairSigner::<IndracoreNodeRuntime, Pair>::new(self.sender.clone());
        async_std::task::block_on(async move {
            let client = match ClientBuilder::<IndracoreNodeRuntime>::new()
                .set_url(primitives::url())
                .build()
                .await
            {
                Ok(cli) => cli,
                Err(e) => return Err(e),
            };

            let hash = match client.transfer(&signer, &self.reciever, self.amount).await {
                Ok(hash) => hash,
                Err(e) => return Err(e),
            };
            Ok(hash)
        })
    }
}
