// Rust libraries are called "crates", and this is how you import them
extern crate num_complex;

// "use" is used to get names into your namespace
use std::ffi::CString;
use std::os::raw::c_char;
use num_complex::Complex;

// Extern blocks like this are used to declare the signature of external symbols and functions
extern { 
    fn fft(data: *mut Complex<f64>, n: usize);
    fn show(message: *const c_char, data: *const Complex<f64>, n: usize);
}


// It is common practice to wrap an "unsafe" C function in "safe" APIs that use Rust datatypes
fn safe_fft(data: &mut [Complex<f64>]) {
    unsafe {
        fft(data.as_mut_ptr(), data.len());
    }
}

fn safe_show(msg: &str, data: &[Complex<f64>]) {
    let msg_cstring = CString::new(msg).unwrap();
    unsafe {
        show(msg_cstring.as_ptr(), data.as_ptr(), data.len());
    }
}

// main() is, unsurprisingly, the entrypoint of our rust project 
fn main() {
    let mut input_data = [
        Complex::new(1.0, 0.0),
        Complex::new(1.0, 0.0),
        Complex::new(1.0, 0.0),
        Complex::new(1.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
        Complex::new(0.0, 0.0),
    ];

    safe_show("Data: ", &input_data);
    safe_fft(&mut input_data);
    safe_show("FFT: ", &input_data);
}
