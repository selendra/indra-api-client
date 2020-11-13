use crate::primitives;
use substrate_subxt::{
    balances::*, sp_core::crypto::AccountId32, system::*, ClientBuilder, Error,
    IndracoreNodeRuntime,
};

pub struct BalanceInfo {
    pub free: u128,
    pub reserved: u128,
    pub misc_frozen: u128,
    pub fee_frozen: u128,
}

pub fn total_issuance() -> Result<u128, Error> {
    async_std::task::block_on(async move {
        let client = match ClientBuilder::<IndracoreNodeRuntime>::new()
            .set_url(primitives::url())
            .build()
            .await
        {
            Ok(cli) => cli,
            Err(e) => return Err(e),
        };
        let total = match client.total_issuance(None).await {
            Ok(total) => total,
            Err(e) => return Err(e),
        };
        Ok(total)
    })
}

pub fn free_balance(id: AccountId32) -> Result<u128, Error> {
    async_std::task::block_on(async move {
        let client = match ClientBuilder::<IndracoreNodeRuntime>::new()
            .set_url(primitives::url())
            .build()
            .await
        {
            Ok(cli) => cli,
            Err(e) => return Err(e),
        };

        let info = match client.account(&id, None).await {
            Ok(info) => info,
            Err(e) => return Err(e),
        };
        Ok(info.data.free)
    })
}

pub fn balance_info(id: AccountId32) -> Result<BalanceInfo, Error> {
    async_std::task::block_on(async move {
        let client = match ClientBuilder::<IndracoreNodeRuntime>::new()
            .set_url(primitives::url())
            .build()
            .await
        {
            Ok(cli) => cli,
            Err(e) => return Err(e),
        };

        let info = match client.account(&id, None).await {
            Ok(info) => info,
            Err(e) => return Err(e),
        };

        Ok(BalanceInfo {
            free: info.data.free,
            misc_frozen: info.data.misc_frozen,
            reserved: info.data.reserved,
            fee_frozen: info.data.fee_frozen,
        })
    })
}
