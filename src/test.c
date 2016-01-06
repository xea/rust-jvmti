#include <stdio.h>
#include <jvmti.h>

//pub extern fn Agent_OnLoad(vm: *mut JavaVM, options: *mut ::libc::c_char, reserved: *mut ::libc::c_void) -> jint
//extern int Agent_OnLoad(void **vm, char *mut, void *reserved);

//     jint (JNICALL *GetEnv)(JavaVM *vm, void **penv, jint version);

jint JNICALL localGetEnv(JavaVM *vm, void **penv, jint version);
jint JNICALL localDestroyJavaVM(JavaVM *vm);
jvmtiError localAddCapabilities(const jvmtiCapabilities* capabilities_ptr);
extern int test_call(int (*fptr)());
int test_callback();

struct jvmtiInterface_1_ env;
jvmtiEnv envPtr = (jvmtiEnv) &env;

int main(int argc, char **argv) {
    printf("C: Invoking from CMain\n");

    printf("Testing extern: %p %d\n", &test_callback, test_call(&test_callback));

    struct JNIInvokeInterface_ jvm;
    JavaVM *jvmPtr = (JavaVM *) &jvm;

    env.AddCapabilities = &localAddCapabilities;

    jvm.GetEnv = &localGetEnv;
    jvm.DestroyJavaVM = &localDestroyJavaVM;

    printf("C: GetEnv %p Destroy %p\n", jvm.GetEnv, jvm.DestroyJavaVM);
    printf("C: jvmPtr %p jvm %p\n", &jvmPtr, jvmPtr);

    Agent_OnLoad(&jvmPtr, NULL, NULL);

    return 0;
}

jint JNICALL localGetEnv(JavaVM *vm, void **penv, jint version) {
    printf("C: GetEnv PENV %p\n", penv);
    printf("C: GetEnv %p\n", &env);

    printf("C: ENV * %p %p\n", envPtr, *envPtr);
    *penv = envPtr;

    return 0;
}

jint JNICALL localDestroyJavaVM(JavaVM *vm) {
    printf("C: DestroyJavaVM\n");
    return 0;
}

jvmtiError localAddCapabilities(const jvmtiCapabilities* capabilities_ptr) {
    printf("C: Add capabilities");
    return 0;
}

int test_callback() {
    return 17;
}
