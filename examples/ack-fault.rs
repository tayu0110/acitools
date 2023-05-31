use acitools::Client;
use acitools::FaultInst;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let str = include_str!("../sandbox.txt")
        .split_whitespace()
        .collect::<Vec<&str>>();
    let (username, endpoint, password) = (str[0], str[1], str[2]);

    let mut client = Client::new(&username, &endpoint, "", &password).await?;

    let faults = FaultInst::get(&mut client).await?;

    for i in 0..faults.len() {
        faults[i].ack(&mut client).await?;
    }

    let routes = client.get("class/uribv6Route.json")?.send().await?;
    eprintln!("{:#?}", routes);

    Ok(())
}
