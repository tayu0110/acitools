// https://pubhub.devnetcloud.com/media/model-doc-521/docs/doc/jsonmeta/fv/BD.json

enum Property {
    OptimezeWanBandwidth(Option<bool>),     // no(default), false, true
    Annotation(Option<String>),             // min: 0, max: 128, regex: ^[a-zA-Z0-9_.:-]+$
    ArpFlood(Option<bool>),                 // no(default), false, true
    ChildAction(Option<u16>),               // 16384(DeleteAll), 8192(DeleteNoPresent), 4096(Ignore)
    ConfigIssues(Option<u8>),               // Bitmask (default: 0b0000)
                                            // 1 << 0: IGMP Snoop disabled on multicast enabled BD.
                                            // 1 << 1: First Hop Security enabled on layer 2 only bridge domain.
                                            // 1 << 2: L3 Mcast is enabled on vrf where BD has IP learn disable.
                                            // 1 << 3: BD cannot combine hardware proxy and flood in encapsulation.
    Descr(Option<String>),                  // min: 0, max: 128, regex: ^[a-zA-Z0-9\\\\!#$()*,-./:;@ _{|}~?&+]+$
    EpClear(Option<bool>),                  // no(default), false, true
    EpMoveDetectMode(Option<u8>),           // no(default), 1(Detection through GARP)
    HostBasedRouting(Option<bool>),         // no(default), false, true
    IntersiteBumTrafficAllow(Option<bool>), // no(default), false, true
    IntersiteL2Stretch(Option<bool>),       // no(default), false, true
    IpLearning(Option<bool>),               // no(default), false, true
    Ipv6McastAllow(Option<bool>),           // false(default), false, true
    LimitIpLearnToSubnets(Option<bool>),    // true(default), false, true
    LlAddr(Option<std::net::Ipv6Addr>),     // IPv6 Link Local Address
    Mac(Option<mac_address::MacAddress>),   // MAC Address. Default: 00:22:BD:F8:19:FF
    McastAllow(Option<bool>),               // false(default), false, true
    MultiDstPktAct(Option<String>),         // "Flood in BD", "Drop", "Flood in Encapsulation"
    Name(Option<String>),                   // min: 1, max: 64, regex: ^[a-zA-Z0-9_.-]+$
    NameAlias(Option<String>),              // min: 0, max: 63, regex: ^[a-zA-Z0-9_.-]+$
    OwnerKey(Option<String>),               // min: 0, max: 128, regex: ^[a-zA-Z0-9\\\\!#$%()*,-./:;@ _{|}~?&+]+$
    OwnerTag(Option<String>),               // min: 0, max: 64, regex: ^[a-zA-Z0-9\\\\!#$%()*,-./:;@ _{|}~?&+]+$
    Type(Option<String>),                   // "fc", "regular"(default)
    UnicastRoute(Option<bool>),             // true(default), false, true
    UnkMacUcastAct(Option<String>),         // "Flood", "Proxy"(default),
    UnkMcastAct(Option<String>),            // "Flood"(default), "Optimized Flood"
    UserDom(Option<String>),                // min: 0, max: 1024, regex: ^[a-zA-Z0-9_.:-]+$, Default: "all"
    V6unkMcastAct(Option<String>),          // "Flood"(default), "Optimized Flood"
    Vmac(Option<mac_address::MacAddress>),  // Mac Address or "Not Configured"(default)
}

pub struct BridgeDomain {
    properties: Vec<Property>,
    
}