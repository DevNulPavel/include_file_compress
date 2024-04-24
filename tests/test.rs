use include_file_compress::include_file_compress_deflate;
use std::io::Read;

#[test]
fn test_deflate() {
    let decompressed_data = {
        let compressed_content = include_file_compress_deflate!("data_samples/data.txt", 5);
        
        let mut decompressor =
            flate2::read::DeflateDecoder::new(std::io::Cursor::new(compressed_content));
        
        let mut buf = Vec::new();

        decompressor.read_to_end(&mut buf).unwrap();
        
        buf
    };

    let source_data = include_bytes!("../data_samples/data.txt");

    assert_eq!(decompressed_data, source_data);
}
