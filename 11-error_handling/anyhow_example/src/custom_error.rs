use anyhow::{anyhow, bail, Context, Result};

#[derive(Debug)]
pub struct MyError {
    message: String,
}

// anyhow! can only be used with types implement Display and Debug.
impl core::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error occurred {}", self.message)
    }
}

fn from_anyhow() -> Result<()> {
    // `anyhow!` evaluates to an Error
    // from a string, or a format string with arguments
    // or any custom type which implements `Debug` and `Display`.
    Err(anyhow!(MyError {
        message: "from anyhow!".to_string()
    }))
    .with_context(|| "More context added to the error.")
}

fn from_bail() -> Result<()> {
    // `bail!` equals to `return Err(anyhow!($args...))`
    bail!(MyError {
        message: "from bail!".to_string()
    })
}

pub fn test() {
    println!("--- Start module: {}", module_path!());
    match from_anyhow() {
        Ok(_) => (),
        Err(err) => println!("{:?}", err),
    }

    match from_bail() {
        Ok(_) => (),
        Err(err) => println!("{:?}", err),
    }
    println!("--- End module: {}", module_path!());
}
