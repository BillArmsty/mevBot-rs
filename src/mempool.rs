use std::sync::Arc;
// pub mod alert;
use crate::alert;

use ethers::{
    abi::AbiDecode,
    providers::{ Middleware, Provider, StreamExt, TransactionStream, Ws },
};

use crate::address_book::UniV2RouterCalls;

pub async fn loop_mempool(ws_provider: Arc<Provider<Ws>>) {
    // Subscribe on newPendingTransactions.
    let tx_hash_stream = ws_provider.subscribe_pending_txs().await.unwrap();
    let mut tx_stream = TransactionStream::new(&ws_provider, tx_hash_stream, 256);

    // println!("---------- MONITORING MEMPOOL ----------");
    let msg = "---------- MONITORING MEMPOOL ----------";
    let block = 0;
    alert::alert(&msg, &0).await;

    while let Some(maybe_tx) = tx_stream.next().await {
        if let Ok(tx) = maybe_tx {
            if let Ok(decoded) = UniV2RouterCalls::decode(&tx.input) {
                println!("Transaction: {:#?}\nRouter Call: {:#?}\n", tx, decoded);
            }
        }
    }
}
