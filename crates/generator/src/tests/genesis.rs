use crate::genesis::{build_genesis, init_genesis};
use gw_common::{sparse_merkle_tree::H256, state::State};
use gw_config::GenesisConfig;
use gw_store::Store;
use gw_traits::CodeStore;
use gw_types::{
    core::ScriptHashType,
    packed::{HeaderInfo, RollupConfig},
    prelude::*,
};
use std::convert::TryInto;

const GENESIS_BLOCK_HASH: [u8; 32] = [
    134, 64, 72, 150, 137, 64, 233, 158, 25, 221, 158, 131, 245, 62, 114, 206, 215, 194, 47, 153,
    154, 55, 181, 38, 55, 111, 19, 153, 10, 17, 134, 184,
];

#[test]
fn test_init_genesis() {
    let config = GenesisConfig { timestamp: 42 };
    let rollup_config = RollupConfig::default();
    let genesis = build_genesis(&config, &rollup_config).unwrap();
    let genesis_block_hash: [u8; 32] = genesis.genesis.hash();
    assert_eq!(genesis_block_hash, GENESIS_BLOCK_HASH);
    let header_info = HeaderInfo::default();
    let store: Store = Store::open_tmp().unwrap();
    init_genesis(&store, &config, &rollup_config, header_info, H256::zero()).unwrap();
    let db = store.begin_transaction();
    // check init values
    assert_ne!(db.get_block_smt_root().unwrap(), H256::zero());
    assert_ne!(db.get_account_smt_root().unwrap(), H256::zero());
    let tree = db.account_state_tree().unwrap();
    assert!(tree.get_account_count().unwrap() > 0);
    // get reserved account's script
    let meta_contract_script_hash = tree.get_script_hash(0).expect("script hash");
    assert_ne!(meta_contract_script_hash, H256::zero());
    let script = tree.get_script(&meta_contract_script_hash).expect("script");
    let hash_type: ScriptHashType = script.hash_type().try_into().unwrap();
    assert!(hash_type == ScriptHashType::Data);
    let code_hash: [u8; 32] = script.code_hash().unpack();
    assert_ne!(code_hash, [0u8; 32]);
}