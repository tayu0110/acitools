use acitools::{BuilderTrait, Client, FvCtx, FvTenant};

#[tokio::test]
async fn ctx_test() {
    let str = include_str!("../sandbox.txt")
        .split_whitespace()
        .collect::<Vec<&str>>();
    let (user, ip, pass) = (str[0], str[1], str[2]);
    let mut client = Client::new(user, ip, "", pass).await.unwrap();

    FvTenant::builder("test-tenant")
        .create(&mut client)
        .await
        .unwrap();

    let res = FvCtx::builder("test-vrf", "test-tenant")
        .create(&mut client)
        .await;
    assert!(res.is_ok());

    let vrfs = FvCtx::get(&mut client);
    assert!(vrfs.is_ok());
    let vrfs = vrfs.unwrap().send().await;
    assert!(vrfs.is_ok());
    assert!(vrfs
        .unwrap()
        .into_iter()
        .any(|vrf| vrf.name() == "test-vrf"));

    let test_tenant = FvTenant::get(&mut client)
        .unwrap()
        .query_target_filter(acitools::Filter::EQ(
            "fvTenant.name".to_string(),
            "test-tenant".to_string(),
        ))
        .send()
        .await;
    assert!(test_tenant.is_ok());
    let test_tenant = test_tenant.as_ref().unwrap();
    assert!(!test_tenant.is_empty());
    let test_tenant = &test_tenant[0];
    let res = test_tenant.get_vrfs(&mut client).await;
    assert!(res.is_ok());
    assert!(res.unwrap().into_iter().any(|vrf| vrf.name() == "test-vrf"));

    let res = FvCtx::builder("test-vrf", "test-tenant")
        .delete(&mut client)
        .await;
    assert!(res.is_ok());

    let vrfs = FvCtx::get(&mut client);
    assert!(vrfs.is_ok());
    let vrfs = vrfs.unwrap().send().await;
    assert!(vrfs.is_ok());
    assert!(vrfs
        .unwrap()
        .into_iter()
        .all(|vrf| vrf.name() != "test-vrf"));

    FvTenant::builder("test-tenant")
        .delete(&mut client)
        .await
        .unwrap();
}

#[test]
fn ctx_deserialize_test() {
    let json = serde_json::json!({
        "fvCtx": {
            "attributes": {
                "annotation": "",
                "childAction": "",
                "descr": "Rust ACI Tool Test",
                "dn": "uni/tn-test-tenant/ctx-test-vrf",
                "name": "test-vrf",
                "nameAlias": "",
                "ownerKey": "",
                "ownerTag": "",
                "status": "",
                "userdom": ":all:",
                "pcEnfDir": "ingress",
                "bdEnforcedEnable": "no",
                "modTs": "2023-05-31T07:40:01.763-07:00",
                "pcEnfDirUpdated": "yes",
                "lcOwn": "local",
                "pcEnfPref": "enforced",
                "ipDataPlaneLearning": "enabled",
                "knwMcastAct": "permit",
                "uid": "15374",
                "monPolDn": "uni/tn-common/monepg-default",
                "seg": "2424837",
                "vrfIndex": "0",
                "pcTag": "32770",
                "scope": "2424837",
                "extMngdBy": "",
                "vrfId": "0",
            },
        }
    });

    let de = serde_json::from_value::<FvCtx>(json);
    if let Err(e) = de.as_ref() {
        eprintln!("{}", e);
    }
    assert!(de.is_ok());
    let de = de.unwrap();
    eprintln!("{:?}", de);
}
