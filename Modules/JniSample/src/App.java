public class App {
    static {
        // System.loadLibrary("test_library"); // Need Library Name without so or dll extension
        System.load("/home/eveheeero/workspace/_Evh/_Evh/Eveheeero/Modules/JniSample/src/test_library.so"); // With absolute path
    }

    public static native int mul(int a, int b);

    public static void main(String[] args) throws Exception {
        System.out.println("Hello, World!");
        System.out.println("mul(2, 3) = " + mul(2, 3));
    }
}
