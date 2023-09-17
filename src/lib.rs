// Copyright (c) 2023 Nathan Sizemore <nathanrsizemore@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::{io, process::Command, str};

#[derive(Debug, Default, Clone)]
pub struct LsbRelease {
    pub id: String,
    pub desc: String,
    pub version: String,
    pub code_name: String,
}

pub fn info() -> io::Result<LsbRelease> {
    let output = Command::new("lsb_release").args(["-a"]).output()?;
    if !output.status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, unsafe {
            str::from_utf8_unchecked(&output.stderr)
        }));
    }

    let text = unsafe { str::from_utf8_unchecked(&output.stdout) };
    let lines = text.split('\n').collect::<Vec<&str>>();
    let mut info = LsbRelease::default();
    for line in lines {
        let split = line.split('\t').collect::<Vec<&str>>();
        if split.len() != 2 {
            continue;
        }

        if split[0] == "Distributor ID:" {
            info.id = split[1].to_owned();
        } else if split[0] == "Description:" {
            info.desc = split[1].to_owned();
        } else if split[0] == "Release:" {
            info.version = split[1].to_owned();
        } else if split[0] == "Codename:" {
            info.code_name = split[1].to_owned();
        }
    }

    Ok(info)
}
