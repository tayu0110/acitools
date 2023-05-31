#![recursion_limit = "256"]

use acitools::{
    Client, FvBD, FvTenant, L2UnknownUnicast, L3UnknownMulticast, MultiDestinationFlooding,
};
use serde::Serialize;

#[tokio::test]
async fn bd_test() {
    let str = include_str!("../sandbox.txt")
        .split_whitespace()
        .collect::<Vec<&str>>();
    let (user, ip, pass) = (str[0], str[1], str[2]);
    let mut client = Client::new(user, ip, "", pass).await.unwrap();

    FvTenant::builder("test-tenant")
        .create(&mut client)
        .await
        .unwrap();

    let res = FvBD::builder("test-bd", "test-tenant")
        .create(&mut client)
        .await;
    assert!(res.is_ok());

    let bds = FvBD::get(&mut client);
    assert!(bds.is_ok());
    let bds = bds.unwrap().send().await;
    assert!(bds.is_ok());
    assert!(bds.unwrap().into_iter().any(|bd| bd.name() == "test-bd"));

    let res = FvBD::builder("test-bd", "test-tenant")
        .delete(&mut client)
        .await;
    assert!(res.is_ok());

    let bds = FvBD::get(&mut client);
    assert!(bds.is_ok());
    let bds = bds.unwrap().send().await;
    assert!(bds.is_ok());
    assert!(bds.unwrap().into_iter().all(|bd| bd.name() != "test-bd"));

    FvTenant::builder("test-tenant")
        .delete(&mut client)
        .await
        .unwrap();
}

#[test]
fn bd_deserialize_test() {
    let json = serde_json::json!({
        "fvBD": {
            "attributes": {
                "OptimizeWanBandwidth": "no",
                "annotation": "",
                "arpFlood": "no",
                "bcastP": "225.1.105.128",
                "childAction": "",
                "configIssues": "",
                "descr": "",
                "dn": "uni/tn-mgmt/BD-inb",
                "enableRogueExceptMac": "no",
                "epClear": "no",
                "epMoveDetectMode": "",
                "extMngdBy": "",
                "hostBasedRouting": "no",
                "intersiteBumTrafficAllow": "no",
                "intersiteL2Stretch": "no",
                "ipLearning": "yes",
                "ipv6McastAllow": "no",
                "lcOwn": "local",
                "limitIpLearnToSubnets": "yes",
                "llAddr": "::",
                "mac": "00:22:BD:F8:19:FF",
                "mcastARPDrop": "yes",
                "mcastAllow": "no",
                "modTs": "2023-05-26T15:15:44.725-07:00",
                "monPolDn": "uni/tn-common/monepg-default",
                "mtu": "inherit",
                "multiDstPktAct": "bd-flood",
                "name": "inb",
                "nameAlias": "",
                "ownerKey": "",
                "ownerTag": "",
                "pcTag": "32770",
                "scope": "2981888",
                "seg": "16154554",
                "status": "",
                "type": "regular",
                "uid": "0",
                "unicastRoute": "yes",
                "unkMacUcastAct": "proxy",
                "unkMcastAct": "flood",
                "userdom": "all",
                "v6unkMcastAct": "flood",
                "vmac": "not-applicable"
            },
        },
    });

    let de = serde_json::from_value::<FvBD>(json);
    if let Err(e) = de.as_ref() {
        eprintln!("{}", e);
    }
    assert!(de.is_ok());
    let de = de.unwrap();
    eprintln!("{:?}", de);
}

#[test]
fn param_deserialize_test() {
    fn inner<T: Serialize>(data: T) -> String {
        serde_json::json!(data).to_string()
    }
    assert_eq!("\"flood\"", inner(L2UnknownUnicast::Flood));
    assert_eq!("\"proxy\"", inner(L2UnknownUnicast::HardwareProxy));
    assert_eq!("\"flood\"", inner(L3UnknownMulticast::Flood));
    assert_eq!("\"opt-flood\"", inner(L3UnknownMulticast::OptimizeFlood));
    assert_eq!("\"bd-flood\"", inner(MultiDestinationFlooding::FloodInBD));
    assert_eq!("\"drop\"", inner(MultiDestinationFlooding::Drop));
    assert_eq!(
        "\"encap-flood\"",
        inner(MultiDestinationFlooding::FloodInEncapsulation)
    );
}
