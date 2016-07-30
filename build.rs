/*
Copyright (c) 2016 Saurav Sachidanand

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
*/

extern crate gcc;
use std::env;
use std::process::Command;

fn main() {
    let mut cfg = gcc::Config::new();
    let env = env::var("TARGET").unwrap();

    cfg.file("nfd/src/nfd_common.c");

    if env.contains("darwin") {
        cfg.file("nfd/src/nfd_cocoa.m");
        cfg.compile("libnfd.a");
        println!("cargo:rustc-link-lib=framework=AppKit");
    } else if env.contains("windows") {
        cfg.file("nfd/src/nfd_win.cpp");
        cfg.compile("libnfd.a");
        println!("cargo:rustc-link-lib=ole32");
        // MinGW doesn't link it by default
        println!("cargo:rustc-link-lib=uuid");
    } else {
        let pkg_output = Command::new("pkg-config")
            .arg("--cflags")
            .arg("gtk+-3.0")
            .arg("glib-2.0")
            .arg("--libs")
            .arg("glib-2.0")
            .output();
        match pkg_output {
            Ok(output) => {
                let t = String::from_utf8(output.stdout).unwrap();
                let flags = t.split(" ");
                for flag in flags {
                    if flag != "\n" && flag != "" {
                        cfg.flag(flag);
                    }
                }
            }
            _ => (),
        }
        cfg.file("nfd/src/nfd_gtk.c");
        cfg.compile("libnfd.a");
        println!("cargo:rustc-link-lib=gdk-3");
        println!("cargo:rustc-link-lib=gtk-3");
        println!("cargo:rustc-link-lib=glib-2.0");
        println!("cargo:rustc-link-lib=gobject-2.0");
    }
}
