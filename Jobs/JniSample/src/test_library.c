#include "App.h" // Write your code with header
// If could not find jni.h, use -I option to /usr/lib/jvm/<Java version>/include
// If cound not find jni_md.h, use -I option to /usr/lib/jvm/<Java version>/include/linux

// When you Don't know how to write JNI code, see header file, and match arguments.
JNIEXPORT int JNICALL // Publish function need JNIEXPORT keyword
Java_App_mul(JNIEnv* env, jclass cls, int a, int b) // Function name should be Java_<Class>_<Function>
{
  return a * b;
}