use ructe::{Result, Ructe};

fn main() -> Result<()> {
    let metadata = std::env::var("CARGO_PKG_METADATA").unwrap_or_default();
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:warning=Metadata: {}", metadata);

    // Initialize Ructe from the environment
    let mut ructe = Ructe::from_env()?;

    // Compile templates located in the "templates" directory
    ructe.compile_templates("templates")?;

    // Instruct Cargo to rerun the build script if any files in the "templates" directory change
    println!("cargo:rerun-if-changed=templates");

    // Additional configuration can be added here if needed

    Ok(())
}
