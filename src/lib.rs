#![doc = include_str!("../README.md")]

////////////////////////////////////////////////////////////////////////////////////////////////////

use flate2::{write::DeflateEncoder, Compression};
use proc_macro::{Span, TokenStream};
use quote::quote;
use std::{io::Write, path::PathBuf};
use syn::{
    parse::{Parse, ParseStream},
    LitByteStr, LitInt, LitStr,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Procedural macros which in compile time includes file and compresses using `deflate`content.
///
/// Macros returns compressed content of file.
///
/// Takes path to file near your project `Cargo.toml` file.
/// It uses `CARGO_MANIFEST_DIR` build time environment variable.
/// 
/// Example below includes content of `data_samples/data.txt` file.
///
/// ```rust
/// # use include_file_compress::include_file_compress_deflate;
/// include_file_compress_deflate!("data_samples/data.txt", 5);
/// ```
#[proc_macro]
pub fn include_file_compress_deflate(input: TokenStream) -> TokenStream {
    // Params parse
    let params: CompressParams = match syn::parse(input) {
        Ok(ok) => ok,
        Err(err) => {
            // If we have parsing error - return it as it.
            return err.into_compile_error().into();
            // Wrapping alternative
            /* return syn::Error::new(err.span(), format_args!("Params error: {}", err))
            .into_compile_error()
            .into(); */
        }
    };

    // Takes call_site for once
    let call_site = Span::call_site().into();

    // Compression
    let compressed_data = match compress_file_deflate(params) {
        Ok(ok) => ok,
        Err(err) => {
            return syn::Error::new(call_site, format_args!("Compress error: {}", err))
                .into_compile_error()
                .into();
        }
    };

    // Embed bytes
    let embed_bytes = LitByteStr::new(&compressed_data, call_site);

    // Embeddable result
    let result = quote!(#embed_bytes);

    result.into()
}

////////////////////////////////////////////////////////////////////////////////////////////////////

struct CompressParams {
    file_path: PathBuf,

    // TODO: Use validated range type
    compression_level: u8,
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

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, thiserror::Error)]
enum CompressError {
    #[error("io error -> {0}")]
    IO(#[from] std::io::Error),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Compression code
#[cfg(not(feature = "mmap"))]
fn compress_file_deflate(params: CompressParams) -> Result<Vec<u8>, CompressError> {
    // Read all file contents into RAM
    let file_data = std::fs::read(params.file_path)?;

    // Compress data
    let compressed_data = {
        let compression = Compression::new(cast::u32(params.compression_level));
        let mut encoder = DeflateEncoder::new(Vec::new(), compression);
        encoder.write_all(&file_data)?;
        // Используем именно .finish() вместо encoder.flush_finish(),
        // иначе стрим не закроется
        encoder.finish()?
    };

    Ok(compressed_data)
}

/// Compression code
#[cfg(feature = "mmap")]
fn compress_file_deflate(params: CompressParams) -> Result<Vec<u8>, CompressError> {
    let file = std::fs::File::open(params.file_path)?;

    // Use mmap for performance
    let mapped_file_content = unsafe { memmap2::Mmap::map(&file)? };

    // Compress data
    let compressed_data = {
        let compression = Compression::new(cast::u32(params.compression_level));
        let mut encoder = DeflateEncoder::new(Vec::new(), compression);
        encoder.write_all(&mapped_file_content)?;
        // Используем именно .finish() вместо encoder.flush_finish(),
        // иначе стрим не закроется
        encoder.finish()?
    };

    Ok(compressed_data)
}
