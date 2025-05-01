use appdomain::IAppDomain;
use assembly::IAssembly;
use windows::{
    core::{w, Interface, Param, GUID, PWSTR}, Win32::{
        Foundation::{GENERIC_READ, S_OK}, Storage::FileSystem::{GetFileSizeEx, ReadFileEx, FILE_ATTRIBUTE_NORMAL, FILE_CREATION_DISPOSITION, FILE_SHARE_READ}, System::{ClrHosting::{CLRCreateInstance, CLSID_CLRMetaHost, ICLRMetaHost, ICLRRuntimeInfo, ICorRuntimeHost}, Com::{SAFEARRAY, SAFEARRAYBOUND}, Ole::{IRecordInfo, SafeArrayAccessData, SafeArrayCreate, SafeArrayCreateEx, SafeArrayCreateVectorEx, SafeArrayLock, SafeArrayUnaccessData, SafeArrayUnlock}, Variant::VT_UI1, IO::OVERLAPPED}
    }
};
use windows::Win32::Storage::FileSystem::CreateFileW;

mod appdomain;
mod assembly;

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};

#[link(name = "hostfxr")]
unsafe extern "C" {
    unsafe fn hostfxr_initialize_for_runtime_config(
        config_path: *const c_char,
        parameters: *const c_void,
        host_context_handle: *mut *mut c_void,
    ) -> i32;
    
    // 他の必要なhostfxr関数...
}

//fn main() -> Result<(), Box<dyn std::error::Error>> {
//    
//    // 結果の処理...
//    
//    Ok(())
//}

fn get_installed_runtime_versions() -> Vec<(String, ICLRRuntimeInfo)> {
    let clr_meta_host: ICLRMetaHost =
        unsafe { CLRCreateInstance::<ICLRMetaHost>(&CLSID_CLRMetaHost).unwrap() };
    let runtimes = unsafe { clr_meta_host.EnumerateInstalledRuntimes().unwrap() };

    let mut installed_versions = vec![];

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
        installed_versions.push((version, runtime_info));
    }
    
    installed_versions
}

fn main() {
    let installed_versions = get_installed_runtime_versions();

    installed_versions.iter().for_each(|v| {
        if v.0 != "v4.0.30319" { return };
        let CLSID_CorRuntimeHost = GUID::from_values(0xcb2f6723, 0xab3a, 0x11d2, [0x9c, 0x40, 0x00, 0xc0, 0x4f, 0xa3, 0x0a, 0x3e]);
        let cor_runtime_host = unsafe {
             v.1.GetInterface::<ICorRuntimeHost>(&CLSID_CorRuntimeHost as *const GUID).unwrap()
        };

        unsafe { cor_runtime_host.Start().unwrap() };
        let friendly_name = w!("test");
        let evidence = unsafe { cor_runtime_host.CreateEvidence().unwrap() };
        let appdomain = unsafe { cor_runtime_host.CreateDomain(friendly_name, &evidence).unwrap() };

        let appdomain = appdomain.cast::<IAppDomain>().unwrap();

        let exe_path = w!(r"C:\Users\lab\Desktop\RoadLoader\.net4-test-project.exe");
        let hwnd = unsafe { 
            CreateFileW(
                exe_path,
                GENERIC_READ.0,
                FILE_SHARE_READ,
                None,
                FILE_CREATION_DISPOSITION(0x3),
                FILE_ATTRIBUTE_NORMAL,
                None,
            ).unwrap()
        };

        let mut lpfilesize = 0_i64;
        unsafe { GetFileSizeEx(hwnd, &mut lpfilesize).unwrap() };
        let mut buf = vec![0_u8; lpfilesize as usize];
        let mut overlapped = OVERLAPPED::default();

        unsafe { 
            ReadFileEx(
                hwnd,
                Some(&mut buf),
                &mut overlapped,
                None, 
            ).unwrap()
        };

        let mut bounds = SAFEARRAYBOUND {
            cElements: buf.len() as _,
            lLbound: 0,
        };
    
        let safe_array_ptr: *mut SAFEARRAY = unsafe { SafeArrayCreate(VT_UI1, 1, &mut bounds) };
        let mut pv_data: *mut c_void = std::ptr::null_mut();
    
        match unsafe { SafeArrayAccessData(safe_array_ptr, &mut pv_data) } {
            Ok(_) => {},
            Err(e) => { println!("e") },
        }
    
        unsafe { std::ptr::copy_nonoverlapping(buf.as_slice().as_ptr(), pv_data.cast(), buf.len()) };
    
        match unsafe { SafeArrayUnaccessData(safe_array_ptr) } {
            Ok(_) => {},
            Err(e) => { println!("e") },
        };

        let mut assembly_ptr = std::ptr::null_mut();
        unsafe { appdomain.Load_3(safe_array_ptr, &mut assembly_ptr).unwrap() };
        println!("{:X?}", assembly_ptr);
        let mut assembly = unsafe { IAssembly::from_raw(assembly_ptr as *mut c_void) };

        println!("{:?}", unsafe { assembly.ToString() });
        // AppDomainのvtblを自力で実装しなきゃいけない
        // .NET9からはLoadFromStreamでAssemblyを読み込める
        // public System.Reflection.Assembly LoadFromStream (System.IO.Stream assembly);
    });
}
