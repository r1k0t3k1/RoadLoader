use clap::{Parser, ValueEnum, command};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CommandLine {
    #[arg(short, long)]
    pub mode: Mode,

    #[arg(short, long)]
    pub url: String,

    #[arg(last = true)]
    pub derive_command: Vec<String>,
}

#[derive(ValueEnum, Clone, Debug)]
enum Mode {
    FileSystem,
    Http,
}

//pub fn get_commandline_args() -> Vec<String> {
//    let mut param_count = 0_i32;
//    let arr_ptr = unsafe {
//        CommandLineToArgvW(GetCommandLineW(), &mut param_count)
//    };
//
//    let args = unsafe { std::slice::from_raw_parts_mut(arr_ptr, param_count as usize) };
//    args.iter().map(|pwstr| {
//        String::from_utf16_lossy(unsafe { pwstr.as_wide() })
//    } ).collect()
//}
