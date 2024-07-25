mod error;
mod plugins;

use clap::Parser;
use plugins::PluginType;
use std::{
    env,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use apigen::{error::Error, from_path, Result};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[clap(value_parser)]
    input: PathBuf,
    #[clap(short, long)]
    output: PathBuf,
    #[clap(short = 't', long, default_value = "path")]
    plugin_type: PluginType,
    #[clap(short, long)]
    plugin_path: String,
}

fn main() {
    let cli = Cli::parse();
    if let Err(err) = from_path(&cli.input).and_then(|spec| run(cli, spec)) {
        report_error(err);
    }
}

fn run(cli: Cli, spec: oas3::Spec) -> Result<()> {
    let spec = serde_json::to_string_pretty(&spec).map_err(|err| Error::Json(err))?;

    let exe = match &cli.plugin_type {
        PluginType::FromSysPath => {
            let exe_name = plugins::plugin_name_from_sys_path(&cli.plugin_path);
            find_on_sys_path(&exe_name)
        }
        PluginType::CustomBinary => Ok(Path::new(&cli.plugin_path).to_path_buf()),
    }?;
    if !exe.is_file() {
        return Err(Error::PluginNotFound(cli.plugin_path));
    }
    let code = run_plugin(&exe, &spec)?;
    save(&cli, &code)
}

fn save(cli: &Cli, code: &str) -> Result<()> {
    std::fs::write(cli.output.as_path(), code).map_err(|err| Error::PluginExec(err))
}

fn run_plugin(exe: &Path, spec: &str) -> Result<String> {
    let mut child = Command::new(exe)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|err| Error::PluginExec(err))?;

    if let Some(stdin) = child.stdin.as_mut() {
        stdin
            .write_all(spec.as_bytes())
            .map_err(|err| Error::Io(err))?;
    }

    let output = child
        .wait_with_output()
        .map_err(|err| Error::PluginExec(err))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::PluginExec(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("[{}] {}", exe.display(), stderr),
        )));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn find_on_sys_path(exe_name: &str) -> Result<PathBuf> {
    let exes = env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths)
            .filter_map(|dir| {
                let full_path = dir.join(exe_name);
                if full_path.is_file() {
                    Some(full_path)
                } else {
                    None
                }
            })
            .next()
    });
    exes.ok_or_else(|| Error::PluginNotFound(exe_name.to_owned()))
}

#[cfg(debug_assertions)]
fn report_error(err: Error) {
    eprintln!("{:?}", err);
}

#[cfg(not(debug_assertions))]
fn report_error(err: Error) {
    eprintln!("{}", err);
}
