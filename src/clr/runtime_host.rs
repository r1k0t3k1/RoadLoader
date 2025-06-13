use windows::Win32::System::ClrHosting::{ICLRRuntimeInfo, ICorRuntimeHost};
use windows_core::{GUID, Interface, w};

use super::core::appdomain::IAppDomain;

pub struct CLRRuntimeHost {
    inner: ICorRuntimeHost,
}

impl CLRRuntimeHost {
    pub fn from(runtime_info: ICLRRuntimeInfo) -> Self {
        let runtime_host = unsafe {
            runtime_info.GetInterface::<ICorRuntimeHost>(&CLRRuntimeHost::IID() as *const GUID)
        };

        // TODO error check
        let ret = Self {
            inner: runtime_host.unwrap(),
        };
        let _ = unsafe { ret.inner.Start() };
        ret
    }

    pub fn create_domain(&self) -> IAppDomain {
        let friendly_name = w!("test");
        let evidence = unsafe { self.inner.CreateEvidence().unwrap() };
        let appdomain = unsafe { self.inner.CreateDomain(friendly_name, &evidence).unwrap() };

        appdomain.cast::<IAppDomain>().unwrap()
    }

    pub fn get_default_domain(&self) -> IAppDomain {
        let appdomain = unsafe { self.inner.GetDefaultDomain().unwrap() };

        appdomain.cast::<IAppDomain>().unwrap()
    }
    
    pub fn IID() -> GUID {
        GUID::from_values(
            0xcb2f6723,
            0xab3a,
            0x11d2,
            [0x9c, 0x40, 0x00, 0xc0, 0x4f, 0xa3, 0x0a, 0x3e],
        )
    }
}

impl Drop for CLRRuntimeHost {
    fn drop(&mut self) {
        let _ = unsafe { self.inner.Stop() };
    }
}
