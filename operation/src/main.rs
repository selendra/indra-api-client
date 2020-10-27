extern crate env_logger;

pub mod command;
use command::operation::operation_cmd;

use substrate_subxt::{ClientBuilder, DefaultNodeRuntime};

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv::dotenv().expect("Failed to read .env file");
    let url = std::env::var("RPC");

    let client = ClientBuilder::<DefaultNodeRuntime>::new()
        .set_url(url.unwrap())
        .build()
        .await?;

    operation_cmd(client).await;
    Ok(())
}
