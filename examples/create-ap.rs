use acitools::{Client, FvAp, FvApEndpoint, QoSClass};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let str = include_str!("../sandbox.txt")
        .split_whitespace()
        .collect::<Vec<&str>>();
    let (username, endpoint, password) = (str[0], str[1], str[2]);
    let client = Client::new(username, endpoint, "", password).await?;

    let mut ap = FvAp::new("test-app", "test-tenant");
    ap.set_descr("Rust ACI Tool Test");
    ap.set_qos_class(QoSClass::Level3);
    let res = ap.create(FvApEndpoint::MoUni, &client).await?;
    eprintln!("{:#?}", res);

    Ok(())
}
