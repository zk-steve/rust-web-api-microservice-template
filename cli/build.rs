use std::{borrow::Cow, process::Command};

/// Generate the `cargo:` key output
pub fn generate_cargo_keys() {
    let commit = if let Ok(hash) = std::env::var("CLI_GIT_COMMIT_HASH") {
        Cow::from(hash.trim().to_owned())
    } else {
        // We deliberately set the length here to `11` to ensure that
        // the emitted hash is always of the same length; otherwise
        // it can (and will!) vary between different build environments.
        match Command::new("git")
            .args(["rev-parse", "--short=11", "HEAD"])
            .output()
        {
            Ok(o) if o.status.success() => {
                let sha = String::from_utf8_lossy(&o.stdout).trim().to_owned();
                Cow::from(sha)
            }
            Ok(o) => {
                let stderr = String::from_utf8_lossy(&o.stderr).trim().to_owned();
                println!(
                    "cargo:warning=Git command failed with status '{}' with message: '{}'",
                    o.status, stderr,
                );
                Cow::from("unknown")
            }
            Err(err) => {
                println!("cargo:warning=Failed to execute git command: {}", err);
                Cow::from("unknown")
            }
        }
    };

    println!("cargo:rustc-env=CLI_COMMIT_HASH={commit}");
    println!("cargo:rustc-env=CLI_IMPL_VERSION={}", get_version(&commit))
}

fn get_version(impl_commit: &str) -> String {
    let commit_dash = if impl_commit.is_empty() { "" } else { "-" };

    format!(
        "{}{}{}",
        std::env::var("CARGO_PKG_VERSION").unwrap_or_default(),
        commit_dash,
        impl_commit
    )
}

fn main() {
    generate_cargo_keys();
}
