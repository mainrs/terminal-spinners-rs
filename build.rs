use anyhow::Result;
use heck::{ShoutySnakeCase, SnakeCase};
use quote::{format_ident, quote};
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs,
    io::{self, Write},
    path::Path,
    process::Command,
};

// Deserialization type.
#[derive(Debug, Deserialize)]
struct JsonSpinnerData {
    pub frames: Vec<String>,
    pub interval: u64,
}

// Deserialization type.
type Spinners = HashMap<String, JsonSpinnerData>;

fn spinner_name_to_const_name(name: impl AsRef<str>) -> String {
    name.as_ref().to_shouty_snake_case()
}

fn main() -> Result<()> {
    // Read in spinner data.
    let file_content = fs::read_to_string("./spinners.json")?;
    let spinners: Spinners = serde_json::from_str(&file_content)?;

    // Build the constants that hold each spinner data.
    let spinner_const_items = spinners
        .iter()
        .map(|(name, data)| {
            let name = format_ident!("{}", spinner_name_to_const_name(name));
            let frames = &data.frames;
            let interval = data.interval;

            quote! {
                pub const #name: crate::SpinnerData<'static> = crate::SpinnerData {
                    frames: &[
                        #(#frames),*
                    ],
                    interval: #interval,
                };
            }
        })
        .collect::<Vec<_>>();

    let module_to_write = quote! {
        #(#spinner_const_items)*
    };

    fs::write("./src/spinners.rs", module_to_write.to_string())?;

    // Generate examples for each spinner.
    let examples = spinners
        .iter()
        .map(|(name, _)| {
            let spinner_name = format_ident!("{}", spinner_name_to_const_name(name));

            let code = quote! {
                use std::{thread, time::Duration};
                use terminal_spinners::{SpinnerBuilder, #spinner_name};

                fn main() {
                    let text = "Loading unicorns";
                    let handle = SpinnerBuilder::new().spinner(&#spinner_name).text(text).start();
                    thread::sleep(Duration::from_secs(3));
                    handle.done();
                }

            };
            (name, code)
        })
        .collect::<Vec<_>>();

    for (name, code) in examples {
        let filename = format!("{}.rs", name.to_snake_case());
        fs::write(Path::new("./examples").join(filename), code.to_string())?;
    }

    // Format the generated source code. rustfmt is a hard-dependency and the script
    // fails otherwise.
    let output = Command::new("cargo").arg("fmt").output()?;
    if !output.status.success() {
        io::stdout().write_all(&output.stdout)?;
        io::stderr().write_all(&output.stderr)?;
        panic!("Failed to invoke rustfmt");
    }

    // Only re-run if the actual spinner data has changed.
    println!("cargo:rerun-if-changed=spinners.json");
    Ok(())
}
