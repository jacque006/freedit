use std::error::Error;

use alloy::{
    contract::{ContractInstance, Interface},
    json_abi::JsonAbi,
    network::Ethereum,
    primitives::{Address, U256},
    providers::{ProviderBuilder, RootProvider},
    transports::http::{Client, Http}
};

pub struct Semaphore {
    contract: ContractInstance<Http<Client>, RootProvider<Http<Client>>, Ethereum>,
}

pub struct SemaphoreProof {
    merkle_tree_depth: U256,
    merkle_tree_root: U256,
    nullifier: U256,
    message: U256,
    scope: U256,
    points: [U256; 8],
}

impl Semaphore {
    // TODO We can get chain_id from provider (get_chain_id), but that will make this async
    // which does not play nicely with LazyLock used for globals in src/lib.rs.
    // There is probably a way to make it work...
    pub fn new(rpc_url: String, chain_id: u64, contract_address: String) -> Result<Self, Box<dyn Error>> {
        // Read in contract ABI. Ideally we would use sol! macro,
        // but ran into parsing errors related to bytecode field
        let abi_path = std::env::current_dir()?.join("static/abi/ISemaphore.json");
        let abi_str = std::fs::read_to_string(abi_path)?;
        let abi: JsonAbi = serde_json::from_str(&abi_str)?;

        let provider = ProviderBuilder::new().on_http(rpc_url.parse()?);

        let addr = Address::parse_checksummed(contract_address, Some(chain_id))?;
        let contract: ContractInstance<Http<Client>, _, Ethereum> =
            ContractInstance::new(addr, provider, Interface::new(abi));

        Ok(Self { contract })
    }

    // TODO Implement
    // TODO Return type should be a Result since rpc call could fail
    pub async fn verify(self/*, group_id: U256, p: SemaphoreProof*/) -> bool {
        // let valid_value = self.contract.function("verifyProof", &[group_id, p])?.call().await;
        // let valid = valid_value.first().unwrap().as_bool().unwrap().0;
        // valid
        unimplemented!();
    }
}
