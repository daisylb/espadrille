// This Source Code Form is subject to the terms of the
// Mozilla Public License, v. 2.0. If a copy of the MPL was not distributed
// with this file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate clap;
extern crate crypto;
extern crate dirs;
use clap::{App, Arg};
use std::process;
mod env;

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

    let spec = env::EnvironmentSpec {
        python_version: env::PythonVersion::Any,
        package_specs: dependencies,
    };
    let ve_path = env::get_venv(spec);
    process::Command::new(ve_path.join("bin").join("python").as_os_str())
        .args(matches.values_of("python_args").unwrap())
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
