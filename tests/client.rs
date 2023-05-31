use acitools::Client;

#[tokio::test]
async fn login_test() {
    let str = include_str!("../sandbox.txt")
        .split_whitespace()
        .collect::<Vec<&str>>();
    let (user, ip, pass) = (str[0], str[1], str[2]);
    let session = Client::new(user, ip, "", pass).await;
    if let Err(e) = session.as_ref() {
        eprintln!("{}", e);
    }
    assert!(session.is_ok());

    let fqdn = str[1];
    let session = Client::new(user, fqdn, "", pass).await;
    if let Err(e) = session.as_ref() {
        eprintln!("{}", e);
    }
    assert!(session.is_ok());
}
