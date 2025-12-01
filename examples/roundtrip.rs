fn main() {
    let ttf = std::fs::read("tests/Roboto.ttf").unwrap();
    println!("Original TTF: {} bytes", ttf.len());

    let woff2 = woofwoof::compress(&ttf, "", 8, true).unwrap();
    std::fs::write("tests/Roboto.woff2", &woff2).unwrap();
    println!("Wrote tests/Roboto.woff2 ({} bytes)", woff2.len());

    let roundtripped = woofwoof::decompress(&woff2).unwrap();
    std::fs::write("tests/Roboto-roundtripped.ttf", &roundtripped).unwrap();
    println!("Wrote tests/Roboto-roundtripped.ttf ({} bytes)", roundtripped.len());
}
