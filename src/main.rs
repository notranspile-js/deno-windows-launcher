/*
 * Copyright 2021, alex at staticlibs.net
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

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