use windows::Win32::{
    Foundation::GENERIC_READ,
    Storage::FileSystem::{
        CreateFileW, FILE_ATTRIBUTE_NORMAL, FILE_CREATION_DISPOSITION, FILE_SHARE_READ,
        GetFileSizeEx, ReadFileEx,
    },
    System::IO::OVERLAPPED,
};
use windows_core::{BSTR, PCWSTR};

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
