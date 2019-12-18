use crypto::digest::Digest;
use crypto::sha3::Sha3;
use dirs;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process;

pub enum PythonVersion {
    Any,
    MinorVersion(u8, u8),
}

pub struct EnvironmentSpec<'a> {
    pub python_version: PythonVersion,
    pub package_specs: Vec<&'a str>,
}

const PYSCRIPT_SUBPATH: &str = "pyscript/envs";

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

pub fn get_venv(spec: EnvironmentSpec) -> io::Result<PathBuf> {
    // Calculate the hash of the spec
    let hash_value = spec.package_specs.join("\0");
    let mut hasher = Sha3::sha3_256();
    hasher.input_str(&hash_value);
    let hash = hasher.result_str();

    // Calculate the path
    let ve_path: PathBuf = dirs::cache_dir().unwrap().join(PYSCRIPT_SUBPATH).join(hash);

    // Create the venv if it doesn't exist
    if !dir_exists(&ve_path)? {
        fs::create_dir_all(&ve_path)?;
        process::Command::new("python3")
            .arg("-m")
            .arg("venv")
            .arg(ve_path.as_os_str())
            .spawn()?
            .wait()?;
        process::Command::new(ve_path.join("bin").join("python").as_os_str())
            .arg("-m")
            .arg("pip")
            .arg("install")
            .args(spec.package_specs)
            .spawn()?
            .wait()?;
    }

    Ok(ve_path)
}
