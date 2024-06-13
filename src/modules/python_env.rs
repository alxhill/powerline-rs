use std::env;
use std::io::Read;
use std::marker::PhantomData;
use std::path::Path;
use std::process::Command;

use super::Module;
use crate::powerline::Separator;
use crate::{Color, Powerline, Style};

pub struct PythonEnv<S: PythonEnvScheme> {
    scheme: PhantomData<S>,
}

pub trait PythonEnvScheme {
    const SEPARATOR: Separator;
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

const PYTHON_VERSION_CMD: &'static str = r#"from sys import version_info as v; print(f"{v.major}.{v.minor}.{v.micro}")"#;

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
                .spawn()
                .ok()
                .and_then(|child| child.stdout)
                .and_then(|mut output| {
                    let mut out = String::new();
                    output.read_to_string(&mut out)
                        .ok()
                        .map(|_| out)
                })
                .unwrap_or("".into());


            powerline.add_segment(
                format!("🐍 {}", venv_name),
                Style::simple(S::PYVENV_FG, S::PYVENV_BG),
            );
            powerline.add_segment(format!("{}", py_ver_str), Style::simple(S::PYVENV_FG, S::PYVENV_BG));
        }
    }
}
