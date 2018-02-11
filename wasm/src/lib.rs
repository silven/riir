#![feature(proc_macro)]

#[macro_use]
extern crate stdweb;
extern crate sha1;

use stdweb::js_export;

#[js_export]
fn sha1( string: &str ) -> String {
    let mut hasher = sha1::Sha1::new();
    hasher.update( string.as_bytes() );
    hasher.digest().to_string()
}


#[js_export]
fn back_and_forth() -> stdweb::Value {
    let message = "Hello, 世界!";
    println!("Cannot print messages from Rust");

    let formatted_message = format!("But we can send them to js: {}", message);

    let result = js! {
        console.log( @{formatted_message} );
        return 2 + 2 * 2;
    };

    return result;
}


