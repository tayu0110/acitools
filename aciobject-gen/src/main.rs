use convert_case::{Case, Casing};
use regex::Regex;
use serde_json::{from_str, Value};
use std::{fs::File, io::Read, process::exit};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut arg = std::env::args();
    let (command, file) = (arg.next().unwrap(), arg.next());

    let Some(file) = file else {
        eprintln!("Usage: {command} ${{SCHEMA}}.json");
        exit(1);
    };

    let mut file = File::open(file)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let mut js = from_str::<Value>(&buf)?;
    let obj_name = js
        .as_object()
        .unwrap()
        .keys()
        .next()
        .unwrap()
        .replace(":", "");
    let js = js
        .as_object_mut()
        .unwrap()
        .into_iter()
        .map(|obj| obj.1.as_object_mut().unwrap())
        .next()
        .unwrap();
    let is_configurable = js["isConfigurable"].as_bool().unwrap();

    // import crates
    println!("use serde::{{Deserialize, Serialize}};");
    println!(
        "use crate::{{AciObject, ConfigStatus, {}EndpointScheme}};",
        if is_configurable {
            "Configurable, "
        } else {
            ""
        }
    );
    println!("use std::borrow::Cow;");
    println!("");

    // define Attributes
    println!("#[derive(Debug, Clone, Deserialize, Serialize)]");
    println!("#[serde(rename_all = \"camelCase\")]");
    println!("pub struct Attributes {{");
    for (name, _attr) in js["properties"].as_object().unwrap() {
        let field = name.to_case(Case::Snake);
        match field.as_str() {
            "status" => {
                println!("    status: ConfigStatus,")
            }
            "type" => {
                println!(
                    "    #[serde(rename = \"type\", skip_serializing_if = \"String::is_empty\")]"
                );
                println!("    r#type: String,")
            }
            "dn" => {
                println!("    #[serde(skip_serializing_if = \"String::is_empty\", default)]");
                println!("    {field}: String,")
            }
            "ext_mnge_by" | "config_issues" | "mon_pol_dn" => {
                println!("    #[allow(dead_code)]");
                println!("    #[serde(skip_serializing)]");
                println!("    {field}: String,")
            }
            _ => {
                if field.to_case(Case::Camel) == name.as_str() {
                    println!("    #[serde(skip_serializing_if = \"String::is_empty\")]");
                } else {
                    println!("    #[serde(skip_serializing_if = \"String::is_empty\", rename = \"{name}\")]");
                }
                println!("    {field}: String,")
            }
        }
    }
    println!("}}");
    println!("");

    // implement Configurable if it is configurable
    if is_configurable {
        println!("impl Configurable for Attributes {{");
        println!("    fn set_status(&mut self, status: ConfigStatus) {{");
        println!("        self.status = status;");
        println!("    }}");
        println!("}}");
        println!("");
    }

    // define ChildItem
    println!("#[derive(Debug, Clone, Deserialize, Serialize)]");
    println!("#[serde(rename_all = \"camelCase\")]");
    println!("pub enum ChildItem {{");
    for name in js["contains"].as_object().unwrap().keys() {
        let name = name.replace(":", "");
        let variant = name.to_case(Case::Pascal);
        if variant.to_case(Case::Camel) != name.as_str() {
            println!("    #[serde(rename = \"{name}\")]")
        }
        println!("    {variant} {{}},");
    }
    println!("}}");
    println!("");

    // define Endpoint
    let re = Regex::new(r#"(([^-/\[\]]+)-\[?)\{([^}]+)\}"#).unwrap();
    let mut endpoints = vec![];
    println!("#[derive(Debug, Clone)]");
    println!("pub enum {}Endpoint {{", obj_name.to_case(Case::Pascal));
    println!("    ClassAll,");
    println!("    MoUni,");
    println!("    Raw(String),");
    for dn in js["dnFormats"]
        .as_array()
        .unwrap()
        .iter()
        .map(|dn| dn.as_str().unwrap())
    {
        let ident = dn
            .split("/")
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .nth(1)
            .unwrap()
            .split("-")
            .next()
            .unwrap()
            .to_case(Case::Pascal);
        let name = format!(
            "Mo{}{}{ident}",
            if dn.starts_with("sys") { "Sys" } else { "" },
            if dn.contains("extch") && ident != "Extch" {
                "ExtCh"
            } else {
                ""
            }
        );
        let is_captured = re.captures(dn).is_some();
        println!("    {name}{}", if is_captured { " {" } else { "" });
        let mut t = vec![];
        let mut pref = vec![];
        for (_, [prefix, class, pt]) in re.captures_iter(dn).map(|m| m.extract()) {
            let name = class.to_case(Case::Snake);
            pref.push((prefix.to_owned(), pt.to_owned(), name.clone()));
            println!("        {name}: String,");
            t.push(name);
        }

        let mut dn = dn.to_owned();
        for (prefix, pt, name) in pref {
            let res = dn.replace(
                format!("{prefix}{{{pt}}}").as_str(),
                format!("{prefix}{{{name}}}").as_str(),
            );
            dn = res.to_string();
        }
        println!("    {},", if is_captured { "}" } else { "" });
        endpoints.push((name, dn, t, is_captured));
    }
    println!("}}");
    println!("");

    // implement EndpointScheme
    println!(
        "impl EndpointScheme for {}Endpoint {{",
        obj_name.to_case(Case::Pascal)
    );
    println!("    fn endpoint(&self) -> Cow<'_, str> {{");
    println!("        match self {{");
    println!("            Self::ClassAll => Cow::Borrowed(\"node/class/{obj_name}.json\"),");
    println!("            Self::MoUni => Cow::Borrowed(\"mo/uni.json\"),");
    println!("            Self::Raw(endpoint) => Cow::Owned(format!(\"{{endpoint}}\")),");
    for (name, dn, t, is_captured) in endpoints {
        println!(
            "            Self::{name}{}",
            if is_captured { " {" } else { "" }
        );
        for t in t {
            println!("                {t},");
        }
        if is_captured {
            println!("            }} => Cow::Owned(format!(\"mo/{dn}.json\")),");
        } else {
            println!("             => Cow::Borrowed(\"mo/{dn}.json\"),");
        }
    }
    println!("        }}");
    println!("    }}");
    println!("}}");
    println!("");

    // define type alias for public
    println!(
        "pub type {0} = AciObject<__internal::{0}>;",
        obj_name.to_case(Case::Pascal)
    );
    println!("");

    // define __internal module
    println!("mod __internal {{");
    println!("    use super::*;");
    println!("    use crate::AciObjectScheme;");
    println!("    #[derive(Debug, Clone, Copy)]");
    println!("    pub struct {};", obj_name.to_case(Case::Pascal));
    println!(
        "    impl AciObjectScheme for {} {{",
        obj_name.to_case(Case::Pascal)
    );
    println!("        type Attributes = Attributes;");
    println!("        type ChildItem = ChildItem;");
    println!(
        "        type Endpoint = {}Endpoint;",
        obj_name.to_case(Case::Pascal)
    );
    println!("        const CLASS_NAME: &'static str = \"{obj_name}\";");
    println!("    }}");
    println!("}}");
    println!("");

    Ok(())
}
