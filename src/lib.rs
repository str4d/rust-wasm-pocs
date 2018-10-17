extern crate cfg_if;
extern crate futures;
extern crate futures_cpupool;
extern crate grpc;
extern crate js_sys;
extern crate pairing;
extern crate protobuf;
extern crate sapling_crypto;
extern crate wasm_bindgen;
extern crate wasm_bindgen_futures;
extern crate zcash_wallet;
extern crate zip32;

mod proto;
mod utils;

use futures::{future, Future, Stream};
use grpc::ClientStubExt;
use pairing::bls12_381::Bls12;
use sapling_crypto::primitives::PaymentAddress;
use std::sync::{Arc, RwLock};
use zcash_wallet::{
    address::encode_payment_address, constants::HRP_SAPLING_EXTENDED_SPENDING_KEY_TEST,
    wallet::WalletTx, welding_rig::scan_block,
};
use zip32::{ExtendedFullViewingKey, ExtendedSpendingKey};

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use proto::server_grpc::CompactTxStreamer;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct Client {
    extfvk: ExtendedFullViewingKey,
    address: PaymentAddress<Bls12>,
    txs: Arc<RwLock<Vec<WalletTx>>>,
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Client {
    pub fn new() -> Self {
        let extsk = ExtendedSpendingKey::master(&[0u8; 32]);
        let extfvk = ExtendedFullViewingKey::from(&extsk);
        let address = extfvk.default_address().unwrap().1;

        Client {
            extfvk,
            address,
            txs: Arc::new(RwLock::new(vec![])),
        }
    }

    pub fn address(&self) -> String {
        encode_payment_address(HRP_SAPLING_EXTENDED_SPENDING_KEY_TEST, &self.address)
    }

    // TODO: This will be inaccurate if the balance exceeds a u32, but u64 -> JavaScript
    // requires BigUint64Array which has limited support across browsers, and is not
    // implemented in the LTS version of Node.js. For now, let's assume that no one is
    // going to use a web wallet with more than ~21 TAZ.
    pub fn balance(&self) -> u32 {
        self.txs.read().unwrap().iter().fold(0, |tx_acc, tx| {
            tx.shielded_outputs
                .iter()
                .fold(tx_acc, |acc, o| acc + o.value)
        }) as u32
    }

    // Returns the number of new transactions detected by the client.
    pub fn sync(&self) -> js_sys::Promise {
        let client = proto::server_grpc::CompactTxStreamerClient::new_plain(
            "127.0.0.1",
            8080,
            Default::default(),
        ).unwrap();

        let mut start = zcash_wallet::welding_rig::block::BlockID::new();
        start.set_blockHeight(280000);

        let mut end = zcash_wallet::welding_rig::block::BlockID::new();
        end.set_blockHeight(300000);

        let mut range = proto::server::RangeFilter::new();
        range.set_start(start);
        range.set_end(end);

        let extfvks = vec![self.extfvk.clone()];
        let txs = self.txs.clone();

        future_to_promise(
            client
                .get_block_range(grpc::RequestOptions::new(), range)
                .drop_metadata()
                .fold::<_, _, future::Ok<_, grpc::Error>>(
                    (0, extfvks, txs),
                    |(acc, extfvks, txs), b| {
                        let mut new_txs = scan_block(b, &extfvks);
                        let count = new_txs.len();
                        txs.write().unwrap().append(&mut new_txs);
                        future::ok((acc + count, extfvks, txs))
                    },
                ).map(|(total, _, _)| JsValue::from(total as u32))
                .map_err(|e| {
                    let js_error = js_sys::Error::new(&format!("uh oh! {:?}", e));
                    JsValue::from(js_error)
                }),
        )
    }
}
