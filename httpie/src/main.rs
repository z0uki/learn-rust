use anyhow::{anyhow, Result};
use reqwest::{Url};

fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;
    Ok(s.into())
}

/// 程序的入口函数，因为在 http 请求时我们使用了异步处理，所以这里引入 tokio
#[tokio::main]
async fn main() -> Result<()> {
    // 判断一下url格式是否正确
    let uri = parse_url("https://www.baidu.com")?;
    // 实例化一个客户端
    let client = reqwest::Client::builder().build()?;
    // 发起get请求 并获取text
    let rsp = client.get(uri).send().await?.text().await?;
    println!("{}", rsp);

    Ok(())
}