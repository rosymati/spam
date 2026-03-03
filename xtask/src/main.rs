use anyhow::{Context, Result, bail};
use sap::{Argument, Parser};

fn main() -> Result<()> {
    let mut parser = Parser::from_env()?;

    while let Some(arg) = parser.forward()? {
        match arg {
            Argument::Value(cmd) => match cmd.as_ref() {
                "generate" => return generate(),
                _ => bail!("Unknown command {cmd}"),
            },
            _ => {}
        }
    }

    Ok(())
}

fn generate() -> Result<()> {
    bindgen::builder()
        .use_core()
        .generate_cstr(true)
        .default_enum_style(bindgen::EnumVariation::NewType {
            is_bitfield: false,
            is_global: false,
        })
        .prepend_enum_name(false)
        .header("headers/headers.h")
        .allowlist_recursively(true)
        .allowlist_type("pam_.*")
        .allowlist_var("pam_.*")
        .allowlist_function("pam_.*")
        .allowlist_type("PAM_.*")
        .allowlist_var("PAM_.*")
        .allowlist_function("PAM_.*")
        .generate()
        .context("Failed to generate bindings")?
        .write_to_file("src/sys.rs")
        .context("Couldn't write bindings")?;

    Ok(())
}
