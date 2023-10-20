use acitools::{Client, FvAp, FvApEndpoint};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let str = include_str!("../sandbox.txt")
        .split_whitespace()
        .collect::<Vec<&str>>();
    let (username, endpoint, password) = (str[0], str[1], str[2]);
    let client = Client::new(username, endpoint, "", password).await?;

    let response = FvAp::get(FvApEndpoint::ClassAll)
        .rsp_subtree(acitools::RspSubTree::Full)
        .send(&client)
        .await?;
    eprintln!("{:#?}", response);

    Ok(())
}
