use acitools::Client;
use acitools::FvCtx;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let str = include_str!("../sandbox.txt")
        .split_whitespace()
        .collect::<Vec<&str>>();
    let (username, endpoint, password) = (str[0], str[1], str[2]);
    let mut client = Client::new(username, endpoint, "", password).await?;

    let res = FvCtx::builder("test-vrf", "test-tenant")
        .set_descr("Rust ACI Tool Test")
        .create(&mut client)
        .await?;

    eprintln!("{:#?}", res);

    Ok(())
}
