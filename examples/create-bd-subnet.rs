use acitools::{Client, FvSubnet};
use std::net::IpAddr::{V4, V6};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let str = include_str!("../sandbox.txt")
        .split_whitespace()
        .collect::<Vec<&str>>();
    let (username, endpoint, password) = (str[0], str[1], str[2]);
    let mut client = Client::new(username, endpoint, "", password).await?;

    let res = FvSubnet::builder(
        V4("192.168.1.254".parse().unwrap()),
        24,
        "test-bd",
        "test-tenant",
    )?
    .set_descr("Rust ACI Tool Test")
    .create(&mut client)
    .await?;
    eprintln!("{:#?}", res);

    let res = FvSubnet::builder(
        V6("2001:db::ffff".parse().unwrap()),
        64,
        "test-bd",
        "test-tenant",
    )?
    .set_descr("Rust ACI Tool Test")
    .create(&mut client)
    .await?;
    eprintln!("{:#?}", res);

    Ok(())
}
