use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=java/main/HelloWorld.java");

    let compilation = Command::new("javac")
        .env("CLASSPATH", "java")
        .args(&["java/main/HelloWorld.java"])
        .status()
        .unwrap();

    assert!(compilation.success());

    let header = Command::new("javah")
        .env("CLASSPATH", "java")
        .args(&["-d", "java", "main.HelloWorld"])
        .status()
        .unwrap();
    
    assert!(header.success());
}