use ethers::prelude::*;
use std::{convert::TryFrom, sync::Arc, time::Duration};
use k256::SecretKey;
use eyre::Result;
use serde::Serialize;

pub struct Node {
    // pub provider: Provider<Http>,
    pub client: Arc<SignerMiddleware<Provider<Http>, LocalWallet>>,
    pub snapshot: U256,
}

#[derive(Clone, Serialize, Debug, PartialEq, Eq, Default)]
pub struct Forking {
    pub json_rpc_url: Option<String>,
    pub block_number: Option<u64>,
}

impl Node {
    pub async fn new() -> Result<Self> {
        let priv_key =
            hex::decode("ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80")?;
        let wallet: LocalWallet = SecretKey::from_be_bytes(&priv_key)
            .expect("did not get private key")
            .into();
        let provider =
            Provider::<Http>::try_from("http://127.0.0.1:8545")?.interval(Duration::from_millis(10u64));
        let client = SignerMiddleware::new(provider, wallet);
        let client = Arc::new(client);

        let snapshot: serde_json::Value = client.inner().request("evm_snapshot", ()).await?;
        let snapshot: U256 = serde_json::from_value::<U256>(snapshot).unwrap();

        Ok(Node{client, snapshot})
    }

    pub async fn snapshot(self: &Self) -> Result<U256> {
        let snapshot: serde_json::Value = self.client.inner().request("evm_snapshot", ()).await?;

        let snapshot: U256 = serde_json::from_value::<U256>(snapshot)?;

        Ok(snapshot)
    }

    pub async fn reset(self: &Self, snapshot: Option<U256>) -> Result<()> {
        let snapshot: U256 = snapshot.map_or(self.snapshot, |x| x);
        let _: serde_json::Value = self.client.inner().request("evm_revert", [snapshot]).await?;

        Ok(())
    }
}

