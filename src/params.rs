use std::path::PathBuf;
use syn::{
    parse::{Parse, ParseStream},
    LitInt, LitStr,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub(super) struct CompressParams {
    pub(super) file_path: PathBuf,

    // TODO: Use validated range type
    pub(super) compression_level: u8,
}

impl Parse for CompressParams {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        // Example: https://github.com/SOF3/include-flate/blob/master/codegen/src/lib.rs

        // File path as first parameter
        let file_path_lit: LitStr = input.parse()?;

        // Check next comma symbol before compression level
        input.parse::<syn::Token![,]>()?;

        // Compression level parse
        let compression_level_lit: LitInt = input.parse()?;

        // Just path + compression level parameter supported now
        if !input.is_empty() {
            // Return error with current input location span
            return Err(
                input.error("Just file path + compression params are supported at this moment")
            );
            // Alternative
            // return Err(syn::Error::new(input.span(), "just file path supported"));
        }

        // Full file path
        let full_file_path = {
            // Takes directory from env variable
            let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").map_err(|err| {
                // Take file path span for error
                syn::Error::new(
                    file_path_lit.span(),
                    format_args!("`CARGO_MANIFEST_DIR` variable is not available: {}", err),
                )
            })?;

            // Join path
            PathBuf::from(manifest_dir).join(file_path_lit.value())
        };

        // Checking if the file exists
        if !full_file_path.exists() {
            // File path span + full file path in error in message
            return Err(syn::Error::new(
                file_path_lit.span(),
                format_args!("File `{}` does not exist", full_file_path.display()),
            ));
        }

        // Base 10 value parse
        let compression_level = compression_level_lit.base10_parse::<u8>()?;

        // Validate compression level value
        if !(1_u8..=9_u8).contains(&compression_level) {
            // Return error with compression level span
            return Err(syn::Error::new(
                compression_level_lit.span(),
                "Compression level must be in range `1..=9`",
            ));
        }

        Ok(CompressParams {
            file_path: full_file_path,
            compression_level,
        })
    }
}
