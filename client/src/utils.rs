use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::signature::Keypair;
use std::sync::Arc;

pub fn create_http_client(server: &str) -> Arc<RpcClient> {
    Arc::new(RpcClient::new(server.into()))
}

pub fn load_dev_wallet() -> Keypair {
    //let pk: [u8; 64] = [1,2,3,4...];
    //Keypair::from_bytes(&pk).unwrap()
    Keypair::new()
}
