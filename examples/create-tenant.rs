use acitools::Client;
use acitools::FvTenant;
use acitools::FvTenantEndpoint;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let str = include_str!("../sandbox.txt")
        .split_whitespace()
        .collect::<Vec<&str>>();
    let (username, endpoint, password) = (str[0], str[1], str[2]);
    let client = Client::new(username, endpoint, "", password).await?;

    let mut tenant = FvTenant::new("test-tenant");
    tenant.set_descr("Test for Rust ACI Tool");
    let response = tenant.create(FvTenantEndpoint::MoUni, &client).await?;
    eprintln!("{:#?}", response);

    Ok(())
}
