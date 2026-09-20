use crate::error::EmberError;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub struct PyExScrLoader {
    venv_dir: PathBuf,
    scripts_dir: PathBuf,
}

impl PyExScrLoader {
    pub fn new() -> Result<Self, EmberError> {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
        let base_dir = home.join(".local/share/ember");
        let venv_dir = base_dir.join("venv");
        let scripts_dir = base_dir.join("pyexscr");

        fs::create_dir_all(&scripts_dir)?;
        Ok(Self {
            venv_dir,
            scripts_dir,
        })
    }

    fn ensure_venv(&self) -> Result<PathBuf, EmberError> {
        let python_bin = if cfg!(windows) {
            self.venv_dir.join("Scripts").join("python.exe")
        } else {
            self.venv_dir.join("bin").join("python")
        };

        if !python_bin.exists() {
            println!("[ember] Initializing sandboxed Python venv in {:?}...", self.venv_dir);
            let status = Command::new("python3")
            .arg("-m")
            .arg("venv")
            .arg(&self.venv_dir)
            .status()
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    EmberError::PythonScriptError(
                        "Host 'python3' binary not found. Python is required to execute local pyexscr extensions.".into()
                    )
                } else {
                    EmberError::Io(e)
                }
            })?;

            if !status.success() {
                return Err(EmberError::PythonScriptError(
                    "Failed to create virtual environment.".into(),
                ));
            }
        }
        Ok(python_bin)
    }

    pub fn execute_extension(&self, module_name: &str, args: &[&str]) -> Result<(), EmberError> {
        let script_file = format!("exscr.ember-{}.py", module_name);
        let script_path = self.scripts_dir.join(&script_file);

        if !script_path.exists() {
            return Err(EmberError::PythonScriptError(format!(
                "Extension script '{}' not found in {:?}",
                script_file, self.scripts_dir
            )));
        }

        let python_bin = self.ensure_venv()?;

        println!("[ember] Running local pyexscr: {}", script_file);
        let status = Command::new(python_bin)
        .arg(&script_path)
        .args(args)
        .status()?;

        if status.success() {
            Ok(())
        } else {
            Err(EmberError::PythonScriptError(format!(
                "Script '{}' exited with code {:?}",
                script_file,
                status.code()
            )))
        }
    }
}
