use alloy::{
    contract::ContractInstance,
    network::Ethereum,
    primitives::{address, U256},
    providers::ProviderBuilder,
    sol,
    transports::http::{Client, Http}
};
// TODO Figure out why this is neede with alloy vs. default Result object
// use eyre::Result;

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    ISemaphore,
    // TODO Figure out why this abi file is not being parsed correctly by this macro
    "static/abi/ISemaphore.json"
);

pub struct Semaphore {
    contract: ContractInstance<Http<Client>, _, Ethereum>,
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
    pub fn new(rpc_url: String, semaphore_contract_address: String) -> Self {
        // TODO may need to Box provider, contract, or this struct so it stays in heap rather than stack
        let provider = ProviderBuilder::new().on_http(rpc_url.parse()?);
        let contract = ISemaphore::new(address!(semaphore_contract_address), provider);

        Self { contract }
    }

        // TODO May need to use U256 instead of uint256 for args when reading from chain
    pub async fn verify(self, group_id: U256, p: SemaphoreProof) -> bool {
        let valid_value = self.contract.function("verifyProof", &[group_id, p])?.call().await;
        let valid = valid_value.first().unwrap().as_bool().unwrap().0;
        valid
    }
}