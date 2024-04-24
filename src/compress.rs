use crate::params::CompressParams;
use flate2::{write::DeflateEncoder, Compression};
use std::io::Write;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, thiserror::Error)]
pub(super) enum CompressError {
    #[error("io error -> {0}")]
    IO(#[from] std::io::Error),
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Compression code
#[cfg(not(feature = "mmap"))]
pub(super) fn compress_file_deflate(params: CompressParams) -> Result<Vec<u8>, CompressError> {
    // Read all file contents into RAM
    let file_data = std::fs::read(params.file_path)?;

    // Compress data
    let compressed_data = deflate_compress_data(&mapped_file_content, params.compression_level)?;

    Ok(compressed_data)
}

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Compression code
#[cfg(feature = "mmap")]
pub(super) fn compress_file_deflate(params: CompressParams) -> Result<Vec<u8>, CompressError> {
    // Opens file
    let file = std::fs::File::open(params.file_path)?;

    // Use mmap for performance
    let mapped_file_content = unsafe { memmap2::Mmap::map(&file)? };

    // Compress data
    let compressed_data = deflate_compress_data(&mapped_file_content, params.compression_level)?;

    // Close mmap
    drop(mapped_file_content);

    // Close file
    drop(file);

    Ok(compressed_data)
}

////////////////////////////////////////////////////////////////////////////////////////////////////

fn deflate_compress_data(data: &[u8], compression_level: u8) -> Result<Vec<u8>, CompressError> {
    let compression = Compression::new(cast::u32(compression_level));

    let mut encoder = DeflateEncoder::new(Vec::new(), compression);

    encoder.write_all(data)?;

    // Use `.finish()` insted of `encoder.flush_finish()`,
    // or stream closes
    let res = encoder.finish()?;

    Ok(res)
}
