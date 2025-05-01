use core::ffi::c_void;

use windows::Win32::System::{Com::SAFEARRAY, Variant::VARIANT};
use windows_core::{IUnknown_Vtbl, HRESULT};

windows_core::imp::define_interface!(IMethodInfo, IMethodInfo_Vtbl, 0xffcc1b5d_ecb8_38dd_9b01_3dc8abc2aa5f);
windows_core::imp::interface_hierarchy!(IMethodInfo, windows_core::IUnknown);
impl IMethodInfo {
    pub unsafe fn ToString(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ToString)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }

    pub unsafe fn Invoke_3(&self, obj: VARIANT, parameters: *mut SAFEARRAY, pRetVal: *mut VARIANT) -> windows_core::Result<()> {
        unsafe { 
            (windows_core::Interface::vtable(self).Invoke_3)(
                windows_core::Interface::as_raw(self),
                obj,
                parameters,
                pRetVal,
            ).ok()
        }
    }
}

#[repr(C)]
pub struct IMethodInfo_Vtbl {
    pub base__: IUnknown_Vtbl,
    pub GetTypeInfoCount: *const c_void,
    pub GetTypeInfo: *const c_void,
    pub GetIDsOfNames: *const c_void,
    pub Invoke: *const c_void,
    pub ToString: unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut u16) -> HRESULT,
    pub Equals: *const c_void,
    pub GetHashCode: *const c_void,
    pub GetType: *const c_void,
    pub get_MemberType: *const c_void,
    pub get_name: *const c_void,
    pub get_DeclaringType: *const c_void,
    pub get_ReflectedType: *const c_void,
    pub GetCustomAttributes: *const c_void,
    pub GetCustomAttributes_2: *const c_void,
    pub IsDefined: *const c_void,
    pub GetParameters:
        unsafe extern "system" fn(this: *mut c_void, pRetVal: *mut *mut SAFEARRAY) -> HRESULT,
    pub GetMethodImplementationFlags: *const c_void,
    pub get_MethodHandle: *const c_void,
    pub get_Attributes: *const c_void,
    pub get_CallingConvention: *const c_void,
    pub Invoke_2: *const c_void,
    pub get_IsPublic: *const c_void,
    pub get_IsPrivate: *const c_void,
    pub get_IsFamily: *const c_void,
    pub get_IsAssembly: *const c_void,
    pub get_IsFamilyAndAssembly: *const c_void,
    pub get_IsFamilyOrAssembly: *const c_void,
    pub get_IsStatic: *const c_void,
    pub get_IsFinal: *const c_void,
    pub get_IsVirtual: *const c_void,
    pub get_IsHideBySig: *const c_void,
    pub get_IsAbstract: *const c_void,
    pub get_IsSpecialName: *const c_void,
    pub get_IsConstructor: *const c_void,
    pub Invoke_3: unsafe extern "system" fn(
        this: *mut c_void,
        obj: VARIANT,
        parameters: *mut SAFEARRAY,
        pRetVal: *mut VARIANT,
    ) -> HRESULT,
    pub get_returnType: *const c_void,
    pub get_ReturnTypeCustomAttributes: *const c_void,
    pub GetBaseDefinition: *const c_void,
}