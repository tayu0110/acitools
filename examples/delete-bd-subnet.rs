use acitools::{Client, FvSubnet, FvSubnetEndpoint};
use std::net::IpAddr::{V4, V6};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let str = include_str!("../sandbox.txt")
        .split_whitespace()
        .collect::<Vec<&str>>();
    let (username, endpoint, password) = (str[0], str[1], str[2]);
    let client = Client::new(username, endpoint, "", password).await?;

    let res = FvSubnet::new_in_bd(
        V4("192.168.1.254".parse().unwrap()),
        24,
        "test-tenant",
        "test-bd",
    )?
    .delete(FvSubnetEndpoint::MoUni, &client)
    .await?;
    eprintln!("{:#?}", res);

    let res = FvSubnet::new_in_bd(
        V6("2001:db::ffff".parse().unwrap()),
        64,
        "test-tenant",
        "test-bd",
    )?
    .delete(FvSubnetEndpoint::MoUni, &client)
    .await?;
    eprintln!("{:#?}", res);

    Ok(())
}
