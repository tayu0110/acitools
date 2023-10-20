use acitools::Client;
use acitools::FvCtx;
use acitools::FvCtxEndpoint;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let str = include_str!("../sandbox.txt")
        .split_whitespace()
        .collect::<Vec<&str>>();
    let (username, endpoint, password) = (str[0], str[1], str[2]);
    let client = Client::new(username, endpoint, "", password).await?;

    let res = FvCtx::new("test-vrf", "test-tenant")
        .delete(FvCtxEndpoint::MoUni, &client)
        .await?;
    eprintln!("{:#?}", res);

    Ok(())
}
