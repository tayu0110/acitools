use acitools::Client;
use acitools::FaultInst;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let str = include_str!("../sandbox.txt")
        .split_whitespace()
        .collect::<Vec<&str>>();
    let (username, endpoint, password) = (str[0], str[1], str[2]);

    let client = Client::new(&username, &endpoint, "", &password).await?;

    let faults = FaultInst::get(&client).await?;

    for i in 0..faults.len() {
        if !faults[i].is_acked() {
            faults[i].ack(&client).await?;
        }
    }

    let routes = client
        .get("node/class/topology/pod-1/topSystem.json")?
        .rsp_subtree(acitools::RspSubTree::FULL)
        .send()
        .await?;
    eprintln!("{:#?}", routes);

    Ok(())
}
