use core::ffi::c_void;

use windows::Win32::System::{
    Com::{SAFEARRAY, SAFEARRAYBOUND},
    Ole::{SafeArrayAccessData, SafeArrayCreate, SafeArrayUnaccessData},
    Variant::VT_UI1,
};
use windows_core::{IUnknown_Vtbl, Interface};

use super::assembly::IAssembly;

windows_core::imp::define_interface!(
    IAppDomain,
    IAppDomain_Vtbl,
    0x05F696DC_2B29_3663_AD8B_C4389CF2A713
);
windows_core::imp::interface_hierarchy!(IAppDomain, windows_core::IUnknown);
impl IAppDomain {
    pub fn load_assembly(&self, assembly: &[u8]) -> IAssembly {
        let mut bounds = SAFEARRAYBOUND {
            cElements: assembly.len() as _,
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

        unsafe { std::ptr::copy_nonoverlapping(assembly.as_ptr(), pv_data.cast(), assembly.len()) };

        match unsafe { SafeArrayUnaccessData(safe_array_ptr) } {
            Ok(_) => {}
            Err(e) => {
                println!("e")
            }
        };

        let mut assembly_ptr = std::ptr::null_mut();
        //unsafe { self.Load_3(safe_array_ptr, &mut assembly_ptr).unwrap() };
        loop {
          match unsafe {self.Load_3(safe_array_ptr, &mut assembly_ptr)} {
            Ok(_) => break,
	        Err(e) => println!("{e}"),
          }
        }  

        unsafe { IAssembly::from_raw(assembly_ptr as *mut c_void) }
    }

    //pub unsafe fn GetTypeInfoCount() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn GetTypeInfo() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn GetIDsOfNames() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn Invoke() -> windows_core::Result<()>{ Ok(()) }
    pub unsafe fn ToString(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ToString)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    //pub unsafe fn Equals() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn GetHashCode() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn GetType() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn InitializeLifetimeService() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn GetLifetimeService() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn Evidence() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn DomainUnload() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn AssemblyLoad() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn ProcessExit() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn TypeResolve() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn ResourceResolve() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn AssemblyResolve() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn UnhandledException() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn DefineDynamicAssembly() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn DefineDynamicAssembly_2() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn DefineDynamicAssembly_3() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn DefineDynamicAssembly_4() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn DefineDynamicAssembly_5() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn DefineDynamicAssembly_6() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn DefineDynamicAssembly_7() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn DefineDynamicAssembly_8() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn DefineDynamicAssembly_9() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn CreateInstance() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn CreateInstanceFrom() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn CreateInstance_2() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn CreateInstanceFrom_2() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn CreateInstance_3() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn CreateInstanceFrom_3() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn Load() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn Load_2() -> windows_core::Result<()>{ Ok(()) }
    pub unsafe fn Load_3(
        &self,
        rawAssembly: *mut SAFEARRAY,
        pRetVal: *mut *mut IAssembly,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Load_3)(
                windows_core::Interface::as_raw(self),
                rawAssembly,
                pRetVal,
            )
            .ok()
        }
    }
    //pub unsafe fn Load_4() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn Load_5() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn Load_6() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn Load_7() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn ExecuteAssembly() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn ExecuteAssembly_2() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn ExecuteAssembly_3() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn FriendlyName() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn BaseDirectory() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn RelativeSearchPath() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn ShadowCopyFiles() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn GetAssemblies() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn AppendPrivatePath() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn ClearPrivatePath() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn SetShadowCopyPath() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn ClearShadowCopyPath() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn SetCachePath() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn SetData() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn GetData() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn SetAppDomainPolicy() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn SetThreadPrincipal() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn SetPrincipalPolicy() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn DoCallBack() -> windows_core::Result<()>{ Ok(()) }
    //pub unsafe fn DynamicDirectory() -> windows_core::Result<()>{ Ok(()) }
}
#[repr(C)]
pub struct IAppDomain_Vtbl {
    pub base__: IUnknown_Vtbl,
    pub GetTypeInfoCount: *const c_void,
    pub GetTypeInfo: *const c_void,
    pub GetIDsOfNames: *const c_void,
    pub Invoke: *const c_void,
    pub ToString: unsafe extern "system" fn(
        this: *mut c_void,
        pRetVal: *mut *mut u16,
    ) -> windows_core::HRESULT,
    pub Equals: *const c_void,
    pub GetHashCode: *const c_void,
    pub GetType: *const c_void,
    pub InitializeLifetimeService: *const c_void,
    pub GetLifetimeService: *const c_void,
    pub get_Evidence: *const c_void,
    pub set_Evidence: *const c_void,
    pub get_DomainUnload: *const c_void,
    pub set_DomainUnload: *const c_void,
    pub get_AssemblyLoad: *const c_void,
    pub set_AssemblyLoad: *const c_void,
    pub get_ProcessExit: *const c_void,
    pub set_ProcessExit: *const c_void,
    pub get_TypeResolve: *const c_void,
    pub set_TypeResolve: *const c_void,
    pub get_ResourceResolve: *const c_void,
    pub set_ResourceResolve: *const c_void,
    pub get_AssemblyResolve: *const c_void,
    pub get_UnhandledException: *const c_void,
    pub set_UnhandledException: *const c_void,
    pub DefineDynamicAssembly: *const c_void,
    pub DefineDynamicAssembly_2: *const c_void,
    pub DefineDynamicAssembly_3: *const c_void,
    pub DefineDynamicAssembly_4: *const c_void,
    pub DefineDynamicAssembly_5: *const c_void,
    pub DefineDynamicAssembly_6: *const c_void,
    pub DefineDynamicAssembly_7: *const c_void,
    pub DefineDynamicAssembly_8: *const c_void,
    pub DefineDynamicAssembly_9: *const c_void,
    pub CreateInstance: *const c_void,
    pub CreateInstanceFrom: *const c_void,
    pub CreateInstance_2: *const c_void,
    pub CreateInstanceFrom_2: *const c_void,
    pub CreateInstance_3: *const c_void,
    pub CreateInstanceFrom_3: *const c_void,
    pub Load: *const c_void,
    pub Load_2: *const c_void,
    pub Load_3: unsafe extern "system" fn(
        this: *mut c_void,
        rawAssembly: *mut SAFEARRAY,
        pRetVal: *mut *mut IAssembly,
    ) -> windows_core::HRESULT,
    pub Load_4: *const c_void,
    pub Load_5: *const c_void,
    pub Load_6: *const c_void,
    pub Load_7: *const c_void,
    pub ExecuteAssembly: *const c_void,
    pub ExecuteAssembly_2: *const c_void,
    pub ExecuteAssembly_3: *const c_void,
    pub get_FriendlyName: *const c_void,
    pub get_BaseDirectory: *const c_void,
    pub get_RelativeSearchPath: *const c_void,
    pub get_ShadowCopyFiles: *const c_void,
    pub GetAssemblies: *const c_void,
    pub AppendPrivatePath: *const c_void,
    pub ClearPrivatePath: *const c_void,
    pub ClearShadowCopyPath: *const c_void,
    pub SetData: *const c_void,
    pub GetData: *const c_void,
    pub SetAppDomainPolicy: *const c_void,
    pub SetThreadPrincipal: *const c_void,
    pub SetPrincipalPolicy: *const c_void,
    pub DoCallBack: *const c_void,
    pub get_DynamicDirectory: *const c_void,
}
