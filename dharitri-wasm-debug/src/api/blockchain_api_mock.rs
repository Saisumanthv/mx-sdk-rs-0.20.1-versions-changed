use crate::TxContext;
use dharitri_wasm::types::{Address, BigUint, DctTokenData, ManagedAddress, TokenIdentifier, H256};

impl dharitri_wasm::api::BlockchainApi for TxContext {
    fn get_sc_address_legacy(&self) -> Address {
        self.tx_input_box.to.clone()
    }

    fn get_owner_address_legacy(&self) -> Address {
        self.blockchain_info_box
            .contract_owner
            .clone()
            .unwrap_or_else(|| panic!("contract owner address not set"))
    }

    fn get_shard_of_address_legacy(&self, _address: &Address) -> u32 {
        panic!("get_shard_of_address not implemented")
    }

    fn is_smart_contract_legacy(&self, _address: &Address) -> bool {
        panic!("is_smart_contract not implemented")

        /*
        Mock used when testing the marketplace contract

        let mut addr_slice = [0u8; 32];
        hex::decode_to_slice(b"6d61726b6574706c6163655f636f6e74726163745f5f5f5f5f5f5f5f5f5f5f5f",
            &mut addr_slice);

        _address == &Address::from_slice(&addr_slice)
        */
    }

    fn get_caller_legacy(&self) -> Address {
        self.tx_input_box.from.clone()
    }

    fn get_balance_legacy(&self, address: &Address) -> BigUint<Self> {
        assert!(
            address == &self.get_sc_address_legacy(),
            "get balance not yet implemented for accounts other than the contract itself"
        );
        self.insert_new_big_uint(self.blockchain_info_box.contract_balance.clone())
    }

    fn get_state_root_hash_legacy(&self) -> H256 {
        panic!("get_state_root_hash_legacy not yet implemented")
    }

    fn get_tx_hash_legacy(&self) -> H256 {
        self.tx_input_box.tx_hash.clone()
    }

    fn get_gas_left(&self) -> u64 {
        self.tx_input_box.gas_limit
    }

    fn get_block_timestamp(&self) -> u64 {
        self.blockchain_info_box.current_block_info.block_timestamp
    }

    fn get_block_nonce(&self) -> u64 {
        self.blockchain_info_box.current_block_info.block_nonce
    }

    fn get_block_round(&self) -> u64 {
        self.blockchain_info_box.current_block_info.block_round
    }

    fn get_block_epoch(&self) -> u64 {
        self.blockchain_info_box.current_block_info.block_epoch
    }

    fn get_block_random_seed_legacy(&self) -> Box<[u8; 48]> {
        self.blockchain_info_box
            .current_block_info
            .block_random_seed
            .clone()
    }

    fn get_prev_block_timestamp(&self) -> u64 {
        self.blockchain_info_box.previous_block_info.block_timestamp
    }

    fn get_prev_block_nonce(&self) -> u64 {
        self.blockchain_info_box.previous_block_info.block_nonce
    }

    fn get_prev_block_round(&self) -> u64 {
        self.blockchain_info_box.previous_block_info.block_round
    }

    fn get_prev_block_epoch(&self) -> u64 {
        self.blockchain_info_box.previous_block_info.block_epoch
    }

    fn get_prev_block_random_seed_legacy(&self) -> Box<[u8; 48]> {
        self.blockchain_info_box
            .previous_block_info
            .block_random_seed
            .clone()
    }

    fn get_current_dct_nft_nonce(
        &self,
        _address: &Address,
        _token: &TokenIdentifier<Self>,
    ) -> u64 {
        // TODO: Implement
        0u64
    }

    // TODO: Include nonce and create a map like: TokenId -> Nonce -> Amount
    fn get_dct_balance(
        &self,
        address: &ManagedAddress<Self>,
        token: &TokenIdentifier<Self>,
        _nonce: u64,
    ) -> BigUint<Self> {
        assert!(
            address == &self.get_sc_address(),
            "get_dct_balance not yet implemented for accounts other than the contract itself"
        );

        match self
            .blockchain_info_box
            .contract_dct
            .get(&token.to_dct_identifier().into_vec())
        {
            Some(value) => self.insert_new_big_uint(value.clone()),
            None => BigUint::zero(self.clone()),
        }
    }

    fn get_dct_token_data(
        &self,
        _address: &ManagedAddress<Self>,
        _token: &TokenIdentifier<Self>,
        _nonce: u64,
    ) -> DctTokenData<Self> {
        panic!("get_dct_token_data not yet implemented")
    }
}