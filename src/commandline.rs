use clap::{builder::{IntoResettable, MapValueParser, NonEmptyStringValueParser, TypedValueParser}, command, Parser, ValueEnum};
use base64::prelude::*;

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

//pub fn get_commandline_args() -> Vec<String> {
//    let mut param_count = 0_i32;
//    let arr_ptr = unsafe {
//        CommandLineToArgvW(GetCommandLineW(), &mut param_count)
//    };
//
//    let args = unsafe { std::slice::from_raw_parts_mut(arr_ptr, param_count as usize) };
//    args.iter().map(|pwstr| {
//        String::from_utf16_lossy(unsafe { pwstr.as_wide() })
//    } ).collect()
//}
