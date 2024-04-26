#![doc = include_str!("../README.md")]

////////////////////////////////////////////////////////////////////////////////////////////////////

mod compress;
mod params;

////////////////////////////////////////////////////////////////////////////////////////////////////

use crate::params::CompressParams;
use compress::compress_file_deflate;
use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::LitByteStr;

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Procedural macros which in compile time includes file and compresses using `deflate`content.
///
/// Macros returns compressed content of file.
///
/// Takes path to file near your project `Cargo.toml` file.
/// It uses `CARGO_MANIFEST_DIR` build time environment variable.
///
/// Parameters:
///
/// - `"data_samples/data.txt"` path to file relative project root
/// - `5` compression level in range 1..=9
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

    // Embed len
    let embed_bytes_len = compressed_data.len();

    // Embed bytes
    let embed_bytes = LitByteStr::new(&compressed_data, call_site);

    // Embeddable result
    let result = quote!({ #embed_bytes as (&'static [u8; #embed_bytes_len]) });

    result.into()
}
