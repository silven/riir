package main;

class HelloWorld {
    // This declares that the static `hello` method will be provided
    // a native library. 
    private static native String hello(String message);

    static {
        try {
			System.loadLibrary("jni_example");
		} catch (UnsatisfiedLinkError e) {
			e.printStackTrace();
			System.exit(1);
		}
    }

    // The rest is just regular ol' java!
    public static void main(String[] args) {
        String output = HelloWorld.hello("Mikael");
        System.out.println(output);
    }
}
