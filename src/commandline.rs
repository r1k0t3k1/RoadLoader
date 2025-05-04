use base64::prelude::*;
use clap::{
    Parser,
    builder::{NonEmptyStringValueParser, TypedValueParser},
    command,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CommandLine {
    #[arg(short, long, group = "location", conflicts_with_all = ["path", "path_b64", "url_b64"])]
    pub url: Option<String>,

    #[arg(short, long, group = "location", conflicts_with_all = ["url", "path_b64", "url_b64"])]
    pub path: Option<String>,

    #[arg(
        long,
        group = "location_b64",
        conflicts_with_all = ["url", "path", "path_b64"],
        value_parser = NonEmptyStringValueParser::new().map(|s| -> String {
            String::from_utf8(BASE64_STANDARD.decode(s).unwrap()).unwrap()
        })
    )]
    pub url_b64: Option<String>,

    #[arg(
        long,
        group = "location_b64", 
        conflicts_with_all = ["url", "path", "url_b64"],
        value_parser = NonEmptyStringValueParser::new().map(|s| -> String {
            String::from_utf8(BASE64_STANDARD.decode(s).unwrap()).unwrap()
        })
    )]
    pub path_b64: Option<String>,

    #[arg(last = true)]
    pub derive_command: Vec<String>,
}
