use std::env;
use std::fs::{read, File};
use std::io::read_to_string;
use std::marker::PhantomData;
use std::path::Path;
use std::process::Command;

use super::Module;
use crate::{Color, Powerline, Style};

pub struct PythonEnv<S: PythonEnvScheme> {
    scheme: PhantomData<S>,
}

pub trait PythonEnvScheme {
    const PYVENV_FG: Color;
    const PYVENV_BG: Color;
    const PYVER_FG: Color;
    const PYVER_BG: Color;
}

impl<S: PythonEnvScheme> Default for PythonEnv<S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: PythonEnvScheme> PythonEnv<S> {
    pub fn new() -> PythonEnv<S> {
        PythonEnv {
            scheme: PhantomData,
        }
    }
}

const PYTHON_VERSION_CMD: &str =
    r#"from sys import version_info as v; print(f"{v.major}.{v.minor}.{v.micro}")"#;
const PYTHON_LOGO: &str = "\u{e73c}";

impl<S: PythonEnvScheme> Module for PythonEnv<S> {
    fn append_segments(&mut self, powerline: &mut Powerline) {
        let venv = env::var("VIRTUAL_ENV")
            .or_else(|_| env::var("CONDA_ENV_PATH"))
            .or_else(|_| env::var("CONDA_DEFAULT_ENV"));

        if let Ok(venv_path) = venv {
            // file_name is always some, because env variable is a valid directory path.
            let venv_name = Path::new(&venv_path).file_name().unwrap().to_string_lossy();

            let py_ver_str = Command::new("python")
                .args(["-c", PYTHON_VERSION_CMD])
                .output()
                .ok()
                .and_then(|output| {
                    std::str::from_utf8(&output.stdout)
                        .map(|s| s.to_owned())
                        .ok()
                })
                .unwrap_or("".into());

            powerline.add_short_segment(
                format!("{} {} ", PYTHON_LOGO, venv_name),
                Style::simple(S::PYVENV_FG, S::PYVENV_BG),
            );
            powerline.add_segment(
                py_ver_str.trim().to_string(),
                Style::simple(S::PYVER_FG, S::PYVER_BG),
            );
        } else if let Some(cwd) = env::current_dir().ok() {
            if cwd.join(".python-version").exists() {
                let py_ver = File::open(cwd.join(".python-version"))
                    .and_then(|f| read_to_string(f))
                    .unwrap_or("UNKNOWN".to_string());
                powerline.add_segment(
                    format!("{} {}", PYTHON_LOGO, py_ver.trim().to_string()),
                    Style::simple(S::PYVER_FG, S::PYVER_BG),
                );
            }
        }
    }
}
