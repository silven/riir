#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// SOURCE_CODE is just a way for me to get access to a large enough amount of data.
static SOURCE_CODE: &'static str = include_str!("./lib.rs");

#[no_mangle]
pub extern fn get_sample_data() -> *const u8 {
    return SOURCE_CODE.as_ptr();
}

#[no_mangle]
pub extern fn get_sample_data_size() -> usize {
    return SOURCE_CODE.len();
}

#[no_mangle]
pub extern fn compress(input: *const u8, output: *mut u8, size: usize) -> u8 {
    unsafe {
        let mut stream: bz_stream = std::mem::zeroed();
        let result = BZ2_bzCompressInit(&mut stream as *mut _,
                                        1,   // 1 x 100000 block size
                                        0,   // verbosity (4 = most verbose)
                                        0);  // default work factor
        match result {
            r if r == (BZ_OK as _) => {},
            _ => return 1,
        }

        stream.next_in = input as *mut _;
        stream.avail_in = size as _;
        stream.next_out = output as *mut _;
        stream.avail_out = size as _;
        let result = BZ2_bzCompress(&mut stream as *mut _, BZ_FINISH as _);
        match result {
            r if r == (BZ_STREAM_END as _) => {},
            _ => return 1,
        }

        let result = BZ2_bzCompressEnd(&mut stream as *mut _);
        match result {
            r if r == (BZ_OK as _) => {},
            _ => return 1,
        }
    }
    return 0;
}


#[no_mangle]
pub extern fn decompress(input: *const u8, output: *mut u8, size: usize) -> u8 {
    unsafe {
         // Construct a decompression stream.
        let mut stream: bz_stream = std::mem::zeroed();
        let result = BZ2_bzDecompressInit(&mut stream as *mut _,
                                        4,   // verbosity (4 = most verbose)
                                        0);  // default small factor
        match result {
            r if r == (BZ_OK as _) => {},
            _ => return 1,
        }

        // Decompress `compressed_output` into `decompressed_output`.
        stream.next_in = input as *mut _;
        stream.avail_in = size as _;
        stream.next_out = output as *mut _;
        stream.avail_out = size as _;
        let result = BZ2_bzDecompress(&mut stream as *mut _);
        match result {
            r if r == (BZ_STREAM_END as _) => {},
            _ => return 1,
        }

        // Close the decompression stream.
        let result = BZ2_bzDecompressEnd(&mut stream as *mut _);
        match result {
            r if r == (BZ_OK as _) => {},
            _ => return 1,
        }
    }
    return 0;
}
