extern crate gcc;

fn main() {
    // The GCC library code used in this build script handles
    // instructing cargo to link with the generated library.
    gcc::Build::new()
           .file("src/fft.c")
           .compile("legacy");
}