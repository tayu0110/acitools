use acitools::{Client, FvSubnet, FvSubnetEndpoint};
use std::net::IpAddr::{V4, V6};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let str = include_str!("../sandbox.txt")
        .split_whitespace()
        .collect::<Vec<&str>>();
    let (username, endpoint, password) = (str[0], str[1], str[2]);
    let mut client = Client::new(username, endpoint, "", password).await?;

    let mut subnet = FvSubnet::new_in_bd(
        V4("192.168.1.254".parse().unwrap()),
        24,
        "test-tenant",
        "test-bd",
    )?;

    subnet.set_descr("Rust ACI Tool Test");
    let res = subnet.create(FvSubnetEndpoint::MoUni, &client).await?;
    eprintln!("{:#?}", res);

    let mut subnet = FvSubnet::new_in_bd(
        V6("2001:db8::ffff".parse().unwrap()),
        64,
        "test-tenant",
        "test-bd",
    )?;
    subnet.set_descr("Rust ACI Tool Test");
    let res = subnet.create(FvSubnetEndpoint::MoUni, &mut client).await?;
    eprintln!("{:#?}", res);

    Ok(())
}
