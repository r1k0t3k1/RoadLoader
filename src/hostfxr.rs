#[link(name = "hostfxr")]
unsafe extern "C" {
    unsafe fn hostfxr_initialize_for_runtime_config(
        config_path: *const c_char,
        parameters: *const c_void,
        host_context_handle: *mut *mut c_void,
    ) -> i32;
    
    // 他の必要なhostfxr関数...
}

//fn main() -> Result<(), Box<dyn std::error::Error>> {
//    
//    // 結果の処理...
//    
//    Ok(())
//}