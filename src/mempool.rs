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
   // let block = 0;
    alert::alert(&msg, &0).await;

    while let Some(maybe_tx) = tx_stream.next().await {
        if let Ok(tx) = maybe_tx {
            if let Ok(decoded) = UniV2RouterCalls::decode(&tx.input) {
                println!("Transaction: {:#?}\nRouter Call: {:#?}\n", tx, decoded);
               
                // let msg = format!("Transaction: {:#?}\nRouter Call: {:#?}\n", tx, decoded);
                let msg = format!("Hash: {:#?}", tx.hash);
                let msg = format!("Gas : {:#?}", tx.gas);
                let msg = format!("Gas Price: {:#?}", tx.gas_price);
                let msg = format!("From: {:#?}", tx.from);
                let msg = format!("To: {:#?}", tx.to);
                let msg = format!("Value: {:#?}", tx.value);
                let msg = format!("maxPriorityFeePerGas: {:#?}", tx.max_priority_fee_per_gas);
                let msg = format!("maxFeePerGas: {:#?}", tx.max_fee_per_gas);
                let msg = format!("ChainId: {:#?}", tx.chain_id);
                // let msg = format!("AmountIn: {:#?}", decoded.);
                let msg = format!("Router Call: {:#?}", decoded);
                // let msg = format!("amountOutMin: {:#?}", decoded.amount_out_min);
                // let msg = format!("path: {:#?}", decoded.path);
                // let msg = format!("to: {:#?}", decoded.to);
                // let msg = format!("deadline: {:#?}", decoded.deadline);
                alert::alert(&msg, &0).await;
            }
        }
    }
}
