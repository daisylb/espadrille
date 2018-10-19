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
use std::process;

fn dir_exists(path: &str) -> io::Result<bool> {
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
                    format!("{} exists and is not a directory", path),
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
        .arg(Arg::with_name("script").takes_value(true).required(true))
        .get_matches();
    let dependencies: Vec<&str> = matches.values_of("dependency").unwrap().collect();
    let hash_value = dependencies.join("\0");
    let mut hasher = Sha3::sha3_256();
    hasher.input_str(&hash_value);
    let hash = hasher.result_str();

    let ve_path = format!(
        "{}/pyscript/envs/{}",
        dirs::cache_dir().unwrap().to_str().unwrap(),
        hash
    );
    if !dir_exists(&ve_path).unwrap() {
        fs::create_dir_all(&ve_path).unwrap();
        process::Command::new("python")
            .arg("-m")
            .arg("virtualenv")
            .arg(&ve_path)
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
        process::Command::new(format!("{}/bin/pip", &ve_path))
            .arg("install")
            .args(dependencies)
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }
    process::Command::new(format!("{}/bin/python", &ve_path))
        .arg(matches.value_of("script").unwrap())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
