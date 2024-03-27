use std::fmt::{Debug, Display};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ReleaseMode {
    Debug,
    Release,
}

impl Display for ReleaseMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Debug => write!(f, "Debug"),
            Self::Release => write!(f, "Release"),
        }
    }
}

pub fn is_dotnet_cli_installed() -> Result<bool, std::io::Error> {
    let output = std::process::Command::new("dotnet")
        .arg("--version")
        .output()?;

    if output.status.success() {
        let version = String::from_utf8_lossy(&output.stdout);
        println!("Le SDK .NET est installé (version {})", version);
        Ok(true)
    } else {
        println!("Le SDK .NET n'est pas installé ou la commande 'dotnet' est introuvable");
        Ok(false)
    }
}

pub fn compile_csharp_assembly(
    project_root_dir: &str,
    release_mode: ReleaseMode,
) -> Result<bool, std::io::Error> {
    match self::is_dotnet_cli_installed() {
        Ok(true) => {
            println!("Lancement de l'analyse de code avec le SDK .NET");

            let output = std::process::Command::new("dotnet")
                .arg("build")
                .arg("-c")
                .arg(release_mode.to_string())
                .arg(project_root_dir)
                .output()?;

            if !output.status.success() {
                println!("Stdout: {}", String::from_utf8_lossy(&output.stdout));

                println!(
                    "Failed to compile C# assembly: {}",
                    String::from_utf8_lossy(&output.stderr)
                );

                return Ok(false);
            }

            Ok(true)
        }
        Ok(false) => Ok(false),
        Err(_) => Ok(false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_dotnet_cli_installed() {
        assert_eq!(self::is_dotnet_cli_installed().unwrap(), true);
    }

    #[test]
    fn test_compile_csharp_assembly() {
        const PROJECT_PATH: &str = r"C:\Users\bubbl\Documents\sumit-app\src\plugins\Finder\client";

        let is_compiled =
            self::compile_csharp_assembly(PROJECT_PATH, ReleaseMode::Release).unwrap();
        assert_eq!(is_compiled, true);
    }
}
