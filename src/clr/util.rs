use std::collections::HashMap;

use windows::Win32::{
    Foundation::S_OK,
    System::{
        ClrHosting::{CLRCreateInstance, CLSID_CLRMetaHost, ICLRMetaHost, ICLRRuntimeInfo},
        Com::SAFEARRAY,
        Ole::{SafeArrayCreateVector, SafeArrayPutElement},
        Variant::{InitVariantFromStringArray, VT_VARIANT},
    },
};
use windows_core::{BSTR, GUID, Interface, PCWSTR, PWSTR};

pub const CLSID_CorRuntimeHost: GUID = GUID::from_values(
    0xcb2f6723,
    0xab3a,
    0x11d2,
    [0x9c, 0x40, 0x00, 0xc0, 0x4f, 0xa3, 0x0a, 0x3e],
);

pub fn get_installed_runtime_versions() -> HashMap<String, ICLRRuntimeInfo> {
    let clr_meta_host: ICLRMetaHost =
        unsafe { CLRCreateInstance::<ICLRMetaHost>(&CLSID_CLRMetaHost).unwrap() };
    let runtimes = unsafe { clr_meta_host.EnumerateInstalledRuntimes().unwrap() };

    let mut installed_versions = HashMap::new();

    let mut buf = [None];
    let mut fetched = 0_u32;
    while unsafe { runtimes.Next(&mut buf, Some(&mut fetched)) } == S_OK {
        let runtime_info = buf[0].clone().unwrap().cast::<ICLRRuntimeInfo>().unwrap();

        let buf = [0_u16; 128];
        let version_string = Some(PWSTR::from_raw(buf.as_ptr() as *mut u16));
        let mut length = 128;
        let _ = unsafe {
            runtime_info
                .GetVersionString(version_string, &mut length)
                .unwrap();
        };
        let version = unsafe { version_string.unwrap().to_string().unwrap() };
        println!("{version}");
        installed_versions.insert(version, runtime_info);
    }

    installed_versions
}

pub fn create_safearray_from_strings(strings: &[String]) -> Result<*mut SAFEARRAY, String> {
    let prgsz = strings
        .iter()
        .map(|s| BSTR::from(s).into_raw())
        .map(|s| PCWSTR(s.clone()))
        .collect::<Vec<PCWSTR>>();

    let v = unsafe { InitVariantFromStringArray(&prgsz).unwrap() };

    let args = vec![v];
    let array_ptr: *mut SAFEARRAY =
        unsafe { SafeArrayCreateVector(VT_VARIANT, 0, args.len() as u32) };

    args.iter().enumerate().for_each(|(i, v)| {
        let v_ref: *const _ = v;
        match unsafe { SafeArrayPutElement(array_ptr, [i as i32].as_ptr(), v_ref as *const _) } {
            Ok(_) => {}
            Err(e) => {
                println!("{e}")
            }
        }
    });

    Ok(array_ptr)
}
