// This Source Code Form is subject to the terms of the
// Mozilla Public License, v. 2.0. If a copy of the MPL was not distributed
// with this file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate clap;
extern crate crypto;
extern crate dirs;
use clap::{App, Arg};
use crypto::digest::Digest;
use crypto::sha3::Sha3;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process;

fn dir_exists(path: &Path) -> io::Result<bool> {
    match fs::metadata(path) {
        Err(e) => {
            if e.kind() == io::ErrorKind::NotFound {
                Ok(false)
            } else {
                Err(e)
            }
        }
        Ok(metadata) => {
            if metadata.is_dir() {
                Ok(true)
            } else {
                Err(io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    format!(
                        "{} exists and is not a directory",
                        path.to_str().unwrap_or("<unprintable path>")
                    ),
                ))
            }
        }
    }
}

fn main() {
    let matches = App::new("pyscript")
        .version("0.1")
        .arg(
            Arg::with_name("dependency")
                .multiple(true)
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("python_args")
                .multiple(true)
                .takes_value(true)
                .last(true),
        )
        .get_matches();
    let dependencies: Vec<&str> = matches.values_of("dependency").unwrap().collect();
    let hash_value = dependencies.join("\0");
    let mut hasher = Sha3::sha3_256();
    hasher.input_str(&hash_value);
    let hash = hasher.result_str();

    let ve_path: PathBuf = dirs::cache_dir()
        .unwrap()
        .join("pyscript")
        .join("envs")
        .join(hash);
    if !dir_exists(&ve_path).unwrap() {
        fs::create_dir_all(&ve_path).unwrap();
        process::Command::new("python3")
            .arg("-m")
            .arg("venv")
            .arg(ve_path.as_os_str())
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
        process::Command::new(ve_path.join("bin").join("pip").as_os_str())
            .arg("install")
            .args(dependencies)
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }
    process::Command::new(ve_path.join("bin").join("python").as_os_str())
        .args(matches.values_of("python_args").unwrap())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
