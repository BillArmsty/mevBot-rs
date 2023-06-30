use std::{sync::Arc, time::Duration};
use crate::alert;

use ethers::prelude::{k256::ecdsa::SigningKey, SignerMiddleware, *};
use tokio::time::sleep;

pub async fn loop_blocks(http_provider: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>) {
    let mut last_block: U64 = U64::zero();
    loop {
        if let Ok(block) = http_provider.get_block_number().await {
            if block > last_block {
                last_block = block;
                // println!("\n---------- BLOCK: {:?} ----------", block);
                let msg = format!("---------- 🔍BLOCK: {:?} ----------", block);
                
                alert::alert(&msg, &0).await;
            }
        }
        sleep(Duration::from_millis(1)).await;
    }
}
