#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryTarget {
    SELF,
    CHILDREN,
    SUBTREE,
}

impl ToString for QueryTarget {
    fn to_string(&self) -> String {
        match self {
            QueryTarget::SELF => "self",
            QueryTarget::CHILDREN => "children",
            QueryTarget::SUBTREE => "subtree",
        }
        .to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassName {
    FvTenant,
    FvCtx,
    FvBD,
    FvSubnet,
    LldpEntity,
    EqptCh,
    EthpmPhysIf,
}

impl ToString for ClassName {
    fn to_string(&self) -> String {
        match self {
            ClassName::FvTenant => "fvTenant",
            ClassName::FvCtx => "fvCtx",
            ClassName::FvBD => "fvBD",
            ClassName::FvSubnet => "fvSubnet",
            ClassName::LldpEntity => "lldpEntity",
            ClassName::EqptCh => "eqptCh",
            ClassName::EthpmPhysIf => "ethpmPhysIf",
        }
        .to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Filter {
    EQ(String, String),
    NE(String, String),
    LT(String, String),
    GT(String, String),
    LE(String, String),
    GE(String, String),
    BW(String, String),
    NOT(Box<Filter>),
    AND(Box<Filter>, Box<Filter>),
    OR(Box<Filter>, Box<Filter>),
    XOR(Box<Filter>, Box<Filter>),
    TRUE,
    FALSE,
    ANYBITS(usize),
    ALLBITS(usize),
    WILDCARD(String),
}

impl ToString for Filter {
    fn to_string(&self) -> String {
        match self {
            Filter::EQ(l, r) => format!("eq({},\"{}\")", l, r),
            Filter::NE(l, r) => format!("ne({},\"{}\")", l, r),
            Filter::LT(l, r) => format!("lt({},\"{}\")", l, r),
            Filter::GT(l, r) => format!("gt({},\"{}\")", l, r),
            Filter::LE(l, r) => format!("le({},\"{}\")", l, r),
            Filter::GE(l, r) => format!("ge({},\"{}\")", l, r),
            Filter::BW(l, r) => format!("bw({},\"{}\")", l, r),
            Filter::NOT(op) => format!("not({})", op.to_string()),
            Filter::AND(l, r) => format!("and({},{})", l.to_string(), r.to_string()),
            Filter::OR(l, r) => format!("or({},{})", l.to_string(), r.to_string()),
            Filter::XOR(l, r) => format!("xor({},{})", l.to_string(), r.to_string()),
            Filter::TRUE => format!("true"),
            Filter::FALSE => format!("false"),
            Filter::ANYBITS(op) => format!("anybits({})", op),
            Filter::ALLBITS(op) => format!("allbits({})", op),
            Filter::WILDCARD(op) => format!("wcard({})", op),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RspSubTree {
    No,
    Children,
    Full,
}

impl ToString for RspSubTree {
    fn to_string(&self) -> String {
        match self {
            RspSubTree::No => "no",
            RspSubTree::Children => "children",
            RspSubTree::Full => "full",
        }
        .to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RspPropInclude {
    All,
    NamingOnly,
    ConfigOnly,
}

impl ToString for RspPropInclude {
    fn to_string(&self) -> String {
        match self {
            RspPropInclude::All => "all",
            RspPropInclude::NamingOnly => "naming-only",
            RspPropInclude::ConfigOnly => "config-only",
        }
        .to_string()
    }
}
