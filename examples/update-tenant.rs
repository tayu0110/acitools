use acitools::Client;
use acitools::{FvTenant, TenantBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let str = include_str!("../sandbox.txt")
        .split_whitespace()
        .collect::<Vec<&str>>();
    let (username, endpoint, password) = (str[0], str[1], str[2]);
    let mut client = Client::new(username, endpoint, "", password).await?;

    let tenants = FvTenant::get(&mut client)?.send().await?;
    for tenant in tenants.into_iter() {
        if tenant.name() == "test-tenant" {
            let response = TenantBuilder::from_tenant(tenant.clone())
                .set_name_alias("test-tenant-alias")
                .update(&mut client)
                .await?;
            eprintln!("{:#?}", response);
        }
    }

    Ok(())
}
