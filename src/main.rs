
#![windows_subsystem = "windows"]

use std::env;
use std::os::windows::process::CommandExt;
use std::process::exit;
use std::process::Command;
use std::process::Stdio;

const CREATE_NO_WINDOW: u32 = 0x08000000;

fn main() {
    let mut exec = env::current_exe().unwrap();
    exec.pop();
    exec.push("deno.exe");
    let mut cmd = Command::new(exec);
    for ar in env::args().skip(1) {
        cmd.arg(ar);
    }
    cmd.stdin(Stdio::null());
    cmd.stdout(Stdio::null());
    cmd.stderr(Stdio::null());
    cmd.creation_flags(CREATE_NO_WINDOW);
    let code = cmd.status().unwrap().code().unwrap();
    exit(code);
}