use std::env;
use std::marker::PhantomData;
use std::path::Path;

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

impl<S: PythonEnvScheme> Module for PythonEnv<S> {
    fn append_segments(&mut self, powerline: &mut Powerline) {
        let venv = env::var("VIRTUAL_ENV")
            .or_else(|_| env::var("CONDA_ENV_PATH"))
            .or_else(|_| env::var("CONDA_DEFAULT_ENV"));

        if let Ok(venv_path) = venv {
            // file_name is always some, because env variable is a valid directory path.
            let venv_name = Path::new(&venv_path).file_name().unwrap().to_string_lossy();

            powerline.add_segment(
                format!("🐍 {}", venv_name),
                Style::simple(S::PYVENV_FG, S::PYVENV_BG),
            )
        }
    }
}
