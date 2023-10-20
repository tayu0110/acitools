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

    let mut tenants = FvTenant::get(FvTenantEndpoint::ClassAll)
        .send(&client)
        .await?;
    for tenant in tenants.iter_mut() {
        if tenant.attributes().name() == "test-tenant" {
            tenant.set_name_alias("test-tenant-alias");
            let response = tenant.update(FvTenantEndpoint::MoUni, &client).await?;
            eprintln!("{:#?}", response);
        }
    }

    Ok(())
}
