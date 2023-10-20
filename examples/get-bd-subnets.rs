use acitools::{Client, FvSubnet, FvSubnetEndpoint};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let str = include_str!("../sandbox.txt")
        .split_whitespace()
        .collect::<Vec<&str>>();
    let (username, endpoint, password) = (str[0], str[1], str[2]);
    let client = Client::new(username, endpoint, "", password).await?;

    let response = FvSubnet::get(FvSubnetEndpoint::ClassTenant {
        tenant: "test-tenant".to_owned(),
    })
    .send(&client)
    .await?;
    eprintln!("{:#?}", response);

    Ok(())
}
