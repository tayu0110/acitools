use acitools::{Client, FvAp, FvTenant};

const TENANT_NAME: &'static str = "test-tenant";
const AP_NAME: &'static str = "test-app-acitools-rust";

#[tokio::test]
async fn bd_test() {
    let str = include_str!("../sandbox.txt")
        .split_whitespace()
        .collect::<Vec<&str>>();
    let (user, ip, pass) = (str[0], str[1], str[2]);
    let client = Client::new(user, ip, "", pass).await.unwrap();

    FvTenant::builder(TENANT_NAME)
        .create(&client)
        .await
        .unwrap();

    let res = FvAp::builder(AP_NAME, TENANT_NAME).create(&client).await;
    assert!(res.is_ok());

    let aps = FvAp::get(&client);
    assert!(aps.is_ok());
    let aps = aps.unwrap().send().await;
    assert!(aps.is_ok());
    assert!(aps.unwrap().into_iter().any(|ap| ap.name() == AP_NAME));

    let res = FvAp::builder(AP_NAME, TENANT_NAME).delete(&client).await;
    assert!(res.is_ok());

    let aps = FvAp::get(&client);
    assert!(aps.is_ok());
    let aps = aps.unwrap().send().await;
    assert!(aps.is_ok());
    eprintln!(
        "aps: {:?}",
        aps.as_ref()
            .unwrap()
            .into_iter()
            .map(|ap| ap.name())
            .collect::<Vec<_>>()
    );
    assert!(aps.unwrap().into_iter().all(|ap| ap.name() != AP_NAME));

    FvTenant::builder(TENANT_NAME)
        .delete(&client)
        .await
        .unwrap();
}

#[test]
fn ap_deserialize_test() {
    let json = serde_json::json!({
        "fvAp": {
            "attributes": {
                "annotation": "",
                "childAction": "",
                "descr": "",
                "dn": "uni/tn-test-tenant/ap-test-app",
                "extMngdBy": "",
                "lcOwn": "local",
                "modTs": "2023-05-31T07:58:39.014-07:00",
                "monPolDn": "uni/tn-common/monepg-default",
                "name": "test-app",
                "nameAlias": "",
                "ownerKey": "",
                "ownerTag": "",
                "prio": "unspecified",
                "status": "",
                "uid": "15374",
                "userdom": ":all:"
            }
        }
    });

    let de = serde_json::from_value::<FvAp>(json);
    if let Err(e) = de.as_ref() {
        eprintln!("{}", e);
    }
    assert!(de.is_ok());
    let de = de.unwrap();
    eprintln!("{:?}", de);
}
