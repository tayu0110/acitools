#![recursion_limit = "256"]

use acitools::{Client, FvTenant};

#[tokio::test]
async fn tenant_test() {
    let str = include_str!("../sandbox.txt")
        .split_whitespace()
        .collect::<Vec<&str>>();
    let (user, ip, pass) = (str[0], str[1], str[2]);
    let mut client = Client::new(user, ip, "", pass).await.unwrap();

    let res = FvTenant::builder("test-tenant").create(&mut client).await;
    assert!(res.is_ok());

    let tenants = FvTenant::get(&mut client);
    assert!(tenants.is_ok());
    let tenants = tenants.unwrap().send().await;
    assert!(tenants.is_ok());
    assert!(tenants
        .unwrap()
        .into_iter()
        .any(|tenant| tenant.name() == "test-tenant"));

    let res = FvTenant::builder("test-tenant").delete(&mut client).await;
    assert!(res.is_ok());

    let tenants = FvTenant::get(&mut client);
    assert!(tenants.is_ok());
    let tenants = tenants.unwrap().send().await;
    assert!(tenants.is_ok());
    assert!(tenants
        .unwrap()
        .into_iter()
        .all(|tenant| tenant.name() != "test-tenant"));
}

#[test]
fn tenant_deserialize_test() {
    let json = serde_json::json!({
        "fvTenant": {
            "attributes": {
                "annotation": "",
                "childAction": "",
                "descr": "Test for Rust ACI Tool",
                "dn": "uni/tn-test-tenant",
                "name": "test-tenant",
                "nameAlias": "",
                "ownerKey": "",
                "ownerTag": "",
                "status": "",
                "lcOwn": "local",
                "uid": "15374",
                "userdom": ":all:",
                "modTs": "2023-05-31T07:31:21.886-07:00",
                "monPolDn": "uni/tn-common/monepg-default",
                "extMngdBy": ""
            }
        }
    });

    let de = serde_json::from_value::<FvTenant>(json);
    if let Err(e) = de.as_ref() {
        eprintln!("{}", e);
    }
    assert!(de.is_ok());
    let de = de.unwrap();
    eprintln!("{:?}", de);
}
