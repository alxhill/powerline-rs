use std::env;
use std::fs::File;
use std::io::read_to_string;
use std::marker::PhantomData;
use std::path::PathBuf;

use crate::{Color, Powerline, Style};
use crate::modules::Module;
use crate::themes::DefaultColors;

pub struct SdkmanJava<S> {
    scheme: PhantomData<S>,
}

pub trait SdkmanScheme: DefaultColors {
    fn sdkman_fg() -> Color {
        Self::default_fg()
    }

    fn sdkman_bg() -> Color {
        Self::default_fg()
    }

    fn icon() -> &'static str {
        "\u{f0176}"
    }
}

impl<S: SdkmanScheme> Default for SdkmanJava<S> {
    fn default() -> Self {
        Self::new()
    }
}
impl<S: SdkmanScheme> SdkmanJava<S> {
    pub fn new() -> SdkmanJava<S> {
        SdkmanJava {
            scheme: PhantomData,
        }
    }
}

impl<S: SdkmanScheme> Module for SdkmanJava<S> {
    fn append_segments(&mut self, powerline: &mut Powerline) {
        let sdkman_env = env::var("SDKMAN_ENV").ok()
            .map(|path| PathBuf::from(path).join(".sdkmanrc"))
            .and_then(|rc_path| File::open(rc_path).ok())
            .and_then(|f| read_to_string(f).ok());

        if let Some(sdkmanrc) = sdkman_env {
            let maybe_java_version = sdkmanrc.lines()
                .filter(|line| !line.starts_with("#"))
                .filter_map(|line| {
                    line.strip_prefix("java=")
                })
                .next();

            if let Some(java_version_str) = maybe_java_version {
                if let Some((version, distribution)) = java_version_str.split_once("-") {
                    let version = if let Some((major_version, _)) = version.split_once(".") {
                        major_version
                    } else {
                        version
                    };

                    powerline.add_segment(
                        format!("{} {} {}", S::icon(), version, distro_name(distribution)),
                        Style::simple(S::sdkman_fg(), S::sdkman_bg()),
                    );
                }
            }
        }
    }
}

fn distro_name(distribution: &str) -> &'static str {
    match distribution {
        "amzn" => "corretto",
        "graal" | "graalce" => "GraalVM",
        "open" => "OpenJDK",
        "zulu" => "Zulu",
        _ => distribution.to_string().leak()
    }
}