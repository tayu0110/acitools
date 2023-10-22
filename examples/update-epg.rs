use acitools::{Client, FvAEPg, FvAEPgEndpoint};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let str = include_str!("../sandbox.txt")
        .split_whitespace()
        .collect::<Vec<&str>>();
    let (username, endpoint, password) = (str[0], str[1], str[2]);
    let client = Client::new(username, endpoint, "", password).await?;

    let mut epg = FvAEPg::new("test-epg", "test-tenant", "test-app");
    epg.set_descr("Updated");
    let res = epg.update(FvAEPgEndpoint::MoUni, &client).await?;
    eprintln!("{:#?}", res);

    let res = FvAEPg::get(FvAEPgEndpoint::MoAp {
        tenant: "test-tenant".to_string(),
        ap: "test-app".to_owned(),
        epg: "test-epg".to_owned(),
    })
    .send(&client)
    .await?;
    eprintln!("{res:#?}");

    Ok(())
}
