#include <jvmti.h>
#include <jni.h>
#include <string.h>

extern JNIEXPORT jint JNICALL R_Agent_OnLoad(JavaVM *vm, char *options, void *reserved);
extern void R_test_call(jvmtiEnv *jvmtiEnv);
static void C_dostuff(jvmtiEnv *jvmti);

static void JNICALL cbVMStart(jvmtiEnv *jvmti, JNIEnv *env);
static void JNICALL cbVMInit(jvmtiEnv *jvmti, JNIEnv *env, jthread thread);
static void JNICALL cbObjectFree(jvmtiEnv *jvmti, jlong tag);
static void JNICALL cbVMObjectAlloc(jvmtiEnv *jvmti, JNIEnv *env, jthread thread, jobject object, jclass object_klass, jlong size);

static jrawMonitorID agent_lock;

extern JNIEXPORT jint JNICALL C_Agent_OnLoad(JavaVM *vm, char *options, void *reserved);

extern JNIEXPORT jint JNICALL C_Agent_OnLoad(JavaVM *vm, char *options, void *reserved) {
//    R_Agent_OnLoad(vm, options, reserved);
    jvmtiEnv *jvmti;
    jvmtiError error;
    jint res;
    jvmtiCapabilities capabilities;
    jvmtiEventCallbacks callbacks;

    res = (*vm)->GetEnv(vm, (void **)&jvmti, JVMTI_VERSION);

    printf("C test: %p %p\n", jvmti, *jvmti);

    jvmtiError (JNICALL *capFnPtr) (jvmtiEnv* env, const jvmtiCapabilities* capabilities_ptr) = (*jvmti)->AddCapabilities;

    printf("Cap ptr: %p\n", capFnPtr);

    R_test_call(jvmti);

    /*
    (void)memset(&capabilities,0, sizeof(capabilities));
    capabilities.can_generate_all_class_hook_events  = 1;
    capabilities.can_tag_objects                     = 1;
    capabilities.can_generate_object_free_events     = 1;
    capabilities.can_get_source_file_name            = 1;
    capabilities.can_get_line_numbers                = 1;
    capabilities.can_generate_vm_object_alloc_events = 1;
    error = (*jvmti)->AddCapabilities(jvmti, &capabilities);

    (void)memset(&callbacks,0, sizeof(callbacks));
    callbacks.VMStart           = &cbVMStart;
    callbacks.VMInit            = &cbVMInit;
//    callbacks.VMDeath           = &cbVMDeath;
    callbacks.ObjectFree        = &cbObjectFree;
    callbacks.VMObjectAlloc     = &cbVMObjectAlloc;
//    callbacks.ClassFileLoadHook = &cbClassFileLoadHook;
    error = (*jvmti)->SetEventCallbacks(jvmti, &callbacks, (jint)sizeof(callbacks));

    error = (*jvmti)->SetEventNotificationMode(jvmti, JVMTI_ENABLE, JVMTI_EVENT_VM_START, (jthread)NULL);
    error = (*jvmti)->SetEventNotificationMode(jvmti, JVMTI_ENABLE, JVMTI_EVENT_VM_INIT, (jthread)NULL);
    error = (*jvmti)->SetEventNotificationMode(jvmti, JVMTI_ENABLE, JVMTI_EVENT_VM_DEATH, (jthread)NULL);
    error = (*jvmti)->SetEventNotificationMode(jvmti, JVMTI_ENABLE, JVMTI_EVENT_OBJECT_FREE, (jthread)NULL);
    error = (*jvmti)->SetEventNotificationMode(jvmti, JVMTI_ENABLE, JVMTI_EVENT_VM_OBJECT_ALLOC, (jthread)NULL);
    error = (*jvmti)->SetEventNotificationMode(jvmti, JVMTI_ENABLE, JVMTI_EVENT_CLASS_FILE_LOAD_HOOK, (jthread)NULL);

    error = (*jvmti)->CreateRawMonitor(jvmti, "agent data", &(agent_lock));
    */

    return JNI_OK;
}

static void JNICALL cbClassFileLoadHook(jvmtiEnv *jvmti, JNIEnv* env,
                jclass class_being_redefined, jobject loader,
                const char* name, jobject protection_domain,
                jint class_data_len, const unsigned char* class_data,
                jint* new_class_data_len, unsigned char** new_class_data) {

}

static void JNICALL cbVMStart(jvmtiEnv *jvmti, JNIEnv *env) {
    printf("VM Start\n");
}

static void JNICALL cbVMInit(jvmtiEnv *jvmti, JNIEnv *env, jthread thread) {
}

static void JNICALL cbObjectFree(jvmtiEnv *jvmti, jlong tag) {
    printf("Object free\n");
}

static void JNICALL cbVMObjectAlloc(jvmtiEnv *jvmti, JNIEnv *env, jthread thread, jobject object, jclass object_klass, jlong size) {
    printf("Object alloc\n");
}

static void C_dostuff(jvmtiEnv *jvmti) {
    
}
