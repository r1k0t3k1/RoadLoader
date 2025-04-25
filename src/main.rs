use windows::{
    core::{w, Interface, Param, GUID, PWSTR}, Win32::{
        Foundation::S_OK,
        System::ClrHosting::{CLRCreateInstance, CLSID_CLRMetaHost, ICLRMetaHost, ICLRRuntimeInfo, ICorRuntimeHost},
    }
};

mod appdomain;

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

        println!("{:?}", appdomain.as_raw());

        //println!("PID: {}", std::process::id());
        //loop {}
        // AppDomainのvtblを自力で実装しなきゃいけない


        // .NET9からはLoadFromStreamでAssemblyを読み込める
        // public System.Reflection.Assembly LoadFromStream (System.IO.Stream assembly);
    });
}
