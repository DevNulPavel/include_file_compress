# Include compress

This procedural macros includes content of file in your binary with compression at compile time.

It can be useful for including static content of CSS, JS files with compression in compile time.

At this moment just `deflate` compression supported only.

# Example

```rust
use include_file_compress::include_file_compress_deflate;

let _compressed_content = include_file_compress_deflate!("data_samples/data.txt", 5);
```
