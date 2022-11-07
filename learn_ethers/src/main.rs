use ethers::prelude::*;
use eyre::Result;


#[tokio::main]
async fn main() -> Result<()> {
    //加载私钥实例化钱包
    let wallet = "a96b7dff3c7e1ddf41dc432f4d4cc97ab82f99c75ea84215f213a2869adec0fa".parse::<LocalWallet>()?;
    println!("{}",wallet.address());

    //连接https rpc
    let http_rpc = Provider::<Http>::connect("https://eth-mainnet.g.alchemy.com/v2/4bLyGWHR628TrMNxI4ERn8ITLsw8jZUL").await;
    //查询blockNumber
    let block_number = http_rpc.get_block(BlockNumber::Latest).await?.unwrap().number.unwrap();
    println!("{}",block_number);

    //连接ws rpc
    let ws_rpc = Provider::<Ws>::connect("wss://eth-mainnet.g.alchemy.com/v2/4bLyGWHR628TrMNxI4ERn8ITLsw8jZUL").await?;

    //实例化一个Filter
    let filter = Filter::new().address("0x3f39ca26c1f4a213e96b3676412a41969ec2bb2a".parse::<Address>().unwrap());

    let mut stream = ws_rpc.subscribe_logs(&filter).await?;

    //循环匹配stream
    while let Some(log) = stream.next().await {
        println!(
            "block: {:?}, tx: {:?}, token: {:?}, from: {:?}, to: {:?}",
            log.block_number,
            log.transaction_hash,
            log.address,
            Address::from(log.topics[1]),
            Address::from(log.topics[2]),
        )
    }

    Ok(())
}