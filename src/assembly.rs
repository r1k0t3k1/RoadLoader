use std::ffi::c_void;

use windows_core::HRESULT;


windows_core::imp::define_interface!(IAssembly, IAssembly_Vtbl, 0x05F696DC_2B29_3663_AD8B_C4389CF2A713);
windows_core::imp::interface_hierarchy!(IAssembly, windows_core::IUnknown);
impl IAssembly { 
    pub unsafe fn ToString(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ToString)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
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
    pub get_FullName: *const c_void,
    pub get_EntryPoint: *const c_void,
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