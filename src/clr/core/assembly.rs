use std::ffi::c_void;

use windows_core::{HRESULT, Interface};

use super::methodinfo::IMethodInfo;

windows_core::imp::define_interface!(
    IAssembly,
    IAssembly_Vtbl,
    0x17156360_2f1a_384a_bc52_fde93c215c5b
);
windows_core::imp::interface_hierarchy!(IAssembly, windows_core::IUnknown);
impl IAssembly {
    pub fn get_entrypoint(&self) -> IMethodInfo {
        let mut entry_ptr = std::ptr::null_mut();
        unsafe { self.get_EntryPoint(&mut entry_ptr).unwrap() };
        unsafe { IMethodInfo::from_raw(entry_ptr as *mut c_void) }
    }

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

    pub unsafe fn get_FullName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FullName)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }

    pub unsafe fn get_EntryPoint(
        &self,
        pRetVal: *mut *mut IMethodInfo,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).get_EntryPoint)(
                windows_core::Interface::as_raw(self),
                pRetVal,
            )
            .ok()
        }
    }
}

#[repr(C)]
pub struct IAssembly_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetTypeInfoCount: *const c_void,
    pub GetTypeInfo: *const c_void,
    pub GetIDsOfNames: *const c_void,
    pub Invoke: *const c_void,
    pub ToString: unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut u16) -> HRESULT,
    pub Equals: *const c_void,
    pub GetHashCode: *const c_void,
    pub GetType: *const c_void,
    pub get_CodeBase: *const c_void,
    pub get_EscapedCodeBase: *const c_void,
    pub GetName: *const c_void,
    pub GetName_2: *const c_void,
    pub get_FullName: unsafe extern "system" fn(
        this: *mut c_void,
        pRetVal: *mut *mut u16,
    ) -> windows_core::HRESULT,
    pub get_EntryPoint: unsafe extern "system" fn(
        this: *mut c_void,
        pRetVal: *mut *mut IMethodInfo,
    ) -> windows_core::HRESULT,
    pub GetType_2: *const c_void,
    pub GetType_3: *const c_void,
    pub GetExportedTypes: *const c_void,
    pub GetTypes: *const c_void,
    pub GetManifestResourceStream: *const c_void,
    pub GetManifestResourceStream_2: *const c_void,
    pub GetFile: *const c_void,
    pub GetFiles: *const c_void,
    pub GetFiles_2: *const c_void,
    pub GetManifestResourceNames: *const c_void,
    pub GetManifestResourceInfo: *const c_void,
    pub get_Location: *const c_void,
    pub get_Evidence: *const c_void,
    pub GetCustomAttributes: *const c_void,
    pub GetCustomAttributes_2: *const c_void,
    pub IsDefined: *const c_void,
    pub GetObjectData: *const c_void,
    pub add_ModuleResolve: *const c_void,
    pub remove_ModuleResolve: *const c_void,
    pub GetType_4: *const c_void,
    pub GetSatelliteAssembly: *const c_void,
    pub GetSatelliteAssembly_2: *const c_void,
    pub LoadModule: *const c_void,
    pub LoadModule_2: *const c_void,
    pub CreateInstance: *const c_void,
    pub CreateInstance_2: *const c_void,
    pub CreateInstance_3: *const c_void,
    pub GetLoadedModules: *const c_void,
    pub GetLoadedModules_2: *const c_void,
    pub GetModules: *const c_void,
    pub GetModules_2: *const c_void,
    pub GetModule: *const c_void,
    pub GetReferencedAssemblies: *const c_void,
    pub get_GlobalAssemblyCache: *const c_void,
}
