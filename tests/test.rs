use include_file_compress::include_file_compress_deflate;
use std::io::Read;

#[test]
fn test_deflate() {
    let source_data = include_bytes!("../data_samples/data.txt");
    // dbg!(source_data.len());

    let decompressed_data = {
        let compressed_content = include_file_compress_deflate!("data_samples/data.txt", 9);
        // dbg!(compressed_content.len());

        let mut decompressor =
            flate2::read::DeflateDecoder::new(std::io::Cursor::new(compressed_content));

        let mut buf = Vec::new();

        decompressor.read_to_end(&mut buf).unwrap();

        buf
    };

    assert_eq!(&decompressed_data, source_data);
}
