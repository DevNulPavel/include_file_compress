use include_file_compress::include_file_compress_deflate;

#[test]
fn test_deflate() {
    let compressed_content = include_file_compress_deflate!("data_samples/data.txt", 5);
}
