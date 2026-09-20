use crate::blueprint::EmberBlueprint;
use crate::error::EmberError;
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::Command;

pub fn execute_build(blueprint: &EmberBlueprint) -> Result<(), EmberError> {
    println!("[ember] Executing build pipeline for '{}'...", blueprint.package.name);

    if let Some(pre_cmds) = &blueprint.build.prebuild {
        for cmd in pre_cmds {
            run_shell_cmd(cmd)?;
        }
    }

    for cmd in &blueprint.build.buildcmd {
        run_shell_cmd(cmd)?;
    }

    if let Some(post_cmds) = &blueprint.build.postbuild {
        for cmd in post_cmds {
            run_shell_cmd(cmd)?;
        }
    }

    if !blueprint.build.artifact.exists() {
        return Err(EmberError::ArtifactNotFound(blueprint.build.artifact.clone()));
    }

    println!(
        "[ember] Build successful. Verified artifact at '{:?}'",
        blueprint.build.artifact
    );
    Ok(())
}

pub fn seal_artifact(blueprint: &EmberBlueprint) -> Result<PathBuf, EmberError> {
    let artifact_path = &blueprint.build.artifact;
    if !artifact_path.exists() {
        return Err(EmberError::ArtifactNotFound(artifact_path.clone()));
    }

    let mut file = File::open(artifact_path)?;
    let mut hasher = Sha256::new();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    hasher.update(&buffer);

    let result = hasher.finalize();
    // Use format_args or slice mapping for GenericArray output formatting
    let hash_result: String = result.iter().map(|b| format!("{:02x}", b)).collect();

    let sidecar_path = PathBuf::from(format!("{}.sha256", artifact_path.display()));
    let mut sidecar_file = File::create(&sidecar_path)?;
    writeln!(
        sidecar_file,
        "{}  {}",
        hash_result,
        artifact_path.file_name().unwrap().to_string_lossy()
    )?;

    println!("[ember] Sealed package artifact. Digest: {}", hash_result);
    Ok(sidecar_path)
}

pub fn remove_workspace(purge_paths: &[PathBuf]) -> Result<(), EmberError> {
    for path in purge_paths {
        if path.exists() {
            if path.is_dir() {
                fs::remove_dir_all(path)?;
            } else {
                fs::remove_file(path)?;
            }
            println!("[ember] Purged target path: {:?}", path);
        }
    }
    Ok(())
}

#[allow(dead_code)]
pub fn douse_workspace(purge_paths: &[PathBuf]) -> Result<(), EmberError> {
    remove_workspace(purge_paths)
}

fn run_shell_cmd(cmd: &str) -> Result<(), EmberError> {
    println!("  > {}", cmd);
    let status = Command::new("sh").arg("-c").arg(cmd).status()?;

    if status.success() {
        Ok(())
    } else {
        Err(EmberError::BuildStepFailed(status.code().unwrap_or(-1)))
    }
}
