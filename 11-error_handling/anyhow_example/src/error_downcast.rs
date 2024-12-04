use anyhow::{anyhow, Context, Result};

#[derive(Debug)]
pub struct MyError {
    message: String,
}

// anyhow! can only be used with types implement Display and Debug.
impl core::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyError occurred {}", self.message)
    }
}

#[derive(Debug)]
pub struct YourError {
    message: String,
}

// anyhow! can only be used with types implement Display and Debug.
impl core::fmt::Display for YourError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "YourError occurred {}", self.message)
    }
}

fn from_anyhow() -> Result<()> {
    // `anyhow!` evaluates to an Error
    // from a string, or a format string with arguments
    // or any custom type which implements `Debug` and `Display`.
    Err(anyhow!(MyError {
        message: "from anyhow!".to_string(),
    }))
    .with_context(|| YourError {
        message: "from anyhow!".to_string(),
    })
}

pub fn test() {
    println!("--- Start module: {}", module_path!());

    match from_anyhow() {
        Ok(_) => (),
        Err(err) => {
            println!("{:?}\n", err);

            if let Some(_) = err.downcast_ref::<MyError>() {
                println!("Error is MyError");
            }

            if let Some(_) = err.downcast_ref::<YourError>() {
                println!("Error is YourError");
            }
        }
    }

    println!("--- End module: {}", module_path!());
}
