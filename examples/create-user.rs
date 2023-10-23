use acitools::Client;
use acitools::{AaaUser, PrivType};
use rpassword::prompt_password;
use std::io::Write;

fn ask() -> (String, String) {
    print!("new username: ");
    std::io::stdout().flush().unwrap();
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let pass = prompt_password("new password: ").unwrap();
    (buf.trim().to_string(), pass)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (new_user, new_pass) = ask();

    let str = include_str!("../sandbox.txt")
        .split_whitespace()
        .collect::<Vec<&str>>();
    let (username, endpoint, password) = (str[0], str[1], str[2]);
    let mut client = Client::new(username, endpoint, "", password).await?;

    let response = AaaUser::new(new_user, new_pass)
        .no_expiration()
        .no_pwd_expiration()
        .add_role("mgmt", "admin", PrivType::WritePriv)
        .add_role("common", "read-all", PrivType::ReadPriv)
        .add_role("common", "admin", PrivType::WritePriv)
        .add_role("all", "admin", PrivType::WritePriv)
        .add_role("all", "read-all", PrivType::ReadPriv)
        .create(&mut client)
        .await?;

    eprintln!("response: {:?}", response);

    Ok(())
}
