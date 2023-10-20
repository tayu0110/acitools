use acitools::{Client, FvCtx, FvCtxEndpoint, FvTenant, FvTenantChild, FvTenantEndpoint};

#[tokio::test]
async fn ctx_test() {
    let str = include_str!("../sandbox.txt")
        .split_whitespace()
        .collect::<Vec<&str>>();
    let (user, ip, pass) = (str[0], str[1], str[2]);
    let client = Client::new(user, ip, "", pass).await.unwrap();

    FvTenant::new("test-tenant")
        .create(FvTenantEndpoint::MoUni, &client)
        .await
        .unwrap();

    let res = FvCtx::new("test-vrf", "test-tenant")
        .create(FvCtxEndpoint::MoUni, &client)
        .await;
    assert!(res.is_ok());

    let vrfs = FvCtx::get(FvCtxEndpoint::ClassTenant {
        tenant: "test-tenant".to_owned(),
    })
    .send(&client)
    .await;
    assert!(vrfs.is_ok());
    assert!(vrfs
        .unwrap()
        .into_iter()
        .any(|vrf| vrf.attributes().name() == "test-vrf"));

    let test_tenant = FvTenant::get(FvTenantEndpoint::ClassAll)
        .query_target_filter(acitools::Filter::EQ(
            "fvTenant.name".to_string(),
            "test-tenant".to_string(),
        ))
        .rsp_subtree(acitools::RspSubTree::Children)
        .send(&client)
        .await;
    assert!(test_tenant.is_ok());
    let test_tenant = test_tenant.as_ref().unwrap();
    assert!(!test_tenant.is_empty());
    let test_tenant = &test_tenant[0];
    let res = test_tenant
        .children()
        .into_iter()
        .filter_map(|c| {
            let FvTenantChild::FvCtx(ctx) = c else { return None };
            Some(ctx)
        })
        .collect::<Vec<_>>();
    assert!(!res.is_empty());
    assert!(res
        .into_iter()
        .any(|vrf| vrf.attributes().name() == "test-vrf"));

    let res = FvCtx::new("test-vrf", "test-tenant")
        .delete(FvCtxEndpoint::MoUni, &client)
        .await;
    assert!(res.is_ok());

    let vrfs = FvCtx::get(FvCtxEndpoint::ClassAll).send(&client).await;
    assert!(vrfs.is_ok());
    assert!(vrfs
        .unwrap()
        .into_iter()
        .all(|vrf| vrf.attributes().name() != "test-vrf"));

    FvTenant::new("test-tenant")
        .delete(FvTenantEndpoint::MoUni, &client)
        .await
        .unwrap();
}

#[test]
fn ctx_deserialize_test() {
    let json = serde_json::json!({
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
    });

    let de = serde_json::from_value::<FvCtx>(json);
    if let Err(e) = de.as_ref() {
        eprintln!("{}", e);
    }
    assert!(de.is_ok());
    let de = de.unwrap();
    eprintln!("{:?}", de);
}
