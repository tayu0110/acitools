mod aciconf;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, help = "Output file format. 'XML' or 'JSON'.")]
    format: Option<String>,
    #[clap(help = "Input filename. Only Yaml format.")]
    filename: String
}

enum OutputFileFormat {
    XML,
    JSON
}
impl OutputFileFormat {
    fn new(f: &str) -> Self {
        match f.to_ascii_uppercase().as_str() {
            "XML" => Self::XML,
            "JSON" => Self::JSON,
            f => {
                eprintln!("Fatal: Internal Error : Unsupported File Format {}.", f);
                std::process::exit(1);
            }
        }
    }
}
impl std::fmt::Debug for OutputFileFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::XML => "XML",
            Self::JSON => "JSON"
        };
        write!(f, "{}", s)
    }
}

fn main() {
    let args = Args::parse();

    let _format = match args.format {
        Some(s) => OutputFileFormat::new(&s),
        None => OutputFileFormat::XML
    };

    let yaml_str = match std::fs::read_to_string(&args.filename) {
        Ok(yaml) => yaml.to_string(),
        Err(e) => panic!("{}", e)
    };

    let yaml_docs = match yaml_rust::YamlLoader::load_from_str(&yaml_str) {
        Ok(yaml_docs) => yaml_docs,
        Err(e) => panic!("{}", e)
    };


}
