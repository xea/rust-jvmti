#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <jvmti.h>
#include <jni.h>

void print_object(void *objPtr, size_t size);
void JNICALL cbVMObjectAlloc(jvmtiEnv *jvmti, JNIEnv *env, jthread thread, jobject object, jclass object_klass, jlong size);
static void JNICALL cbObjectFree(jvmtiEnv *jvmti, jlong tag);
uint32_t test_clib();

uint32_t test_clib() {
    return 14;
}

uint32_t fwd_enable_capabilities(jvmtiEnv *jvmti);

uint32_t fwd_enable_capabilities(jvmtiEnv *jvmti) {
    printf("C: Forward Enable Capabilities\n");
    printf("C: envPtr: %p -> %p\n", jvmti, *jvmti);

    jvmtiCapabilities capabilities;
    jvmtiError error;

    (void)memset(&capabilities,0, sizeof(capabilities));
    capabilities.can_generate_all_class_hook_events  = 1;
    capabilities.can_tag_objects                     = 1;
    capabilities.can_generate_object_free_events     = 1;
    capabilities.can_get_source_file_name            = 1;
    capabilities.can_get_line_numbers                = 1;
    capabilities.can_generate_vm_object_alloc_events = 1;
    error = (*jvmti)->AddCapabilities(jvmti, &capabilities);

    return error;
}

uint32_t fwd_enable_notifications(jvmtiEnv *jvmti);

uint32_t fwd_enable_notifications(jvmtiEnv *jvmti) {
    jvmtiError error;
    jvmtiEventCallbacks callbacks;
    (void)memset(&callbacks,0, sizeof(callbacks));


//    callbacks.VMStart           = &cbVMStart;
//    callbacks.VMInit            = &cbVMInit;
//    callbacks.VMDeath           = &cbVMDeath;
//    callbacks.ObjectFree        = &cbObjectFree;
    callbacks.VMObjectAlloc     = &cbVMObjectAlloc;
//    callbacks.ClassFileLoadHook = &cbClassFileLoadHook;
    print_object(&callbacks, sizeof(callbacks));
    error = (*jvmti)->SetEventCallbacks(jvmti, &callbacks, (jint)sizeof(callbacks));

    printf("C CALLBACK %p\n", &cbVMObjectAlloc);
/*
    error = (*jvmti)->SetEventNotificationMode(jvmti, JVMTI_ENABLE, JVMTI_EVENT_VM_START, (jthread)NULL);
    error = (*jvmti)->SetEventNotificationMode(jvmti, JVMTI_ENABLE, JVMTI_EVENT_VM_INIT, (jthread)NULL);
    error = (*jvmti)->SetEventNotificationMode(jvmti, JVMTI_ENABLE, JVMTI_EVENT_VM_DEATH, (jthread)NULL);
    error = (*jvmti)->SetEventNotificationMode(jvmti, JVMTI_ENABLE, JVMTI_EVENT_OBJECT_FREE, (jthread)NULL);
    error = (*jvmti)->SetEventNotificationMode(jvmti, JVMTI_ENABLE, JVMTI_EVENT_VM_OBJECT_ALLOC, (jthread)NULL);
    error = (*jvmti)->SetEventNotificationMode(jvmti, JVMTI_ENABLE, JVMTI_EVENT_CLASS_FILE_LOAD_HOOK, (jthread)NULL);
    */

    return error;
}

static void JNICALL cbObjectFree(jvmtiEnv *jvmti, jlong tag) {
    printf("Object free\n");
}

void JNICALL cbVMObjectAlloc(jvmtiEnv *jvmti, JNIEnv *env, jthread thread, jobject object, jclass object_klass, jlong size) {
    printf("Object alloc\n");
}

void print_callbacks(jvmtiEventCallbacks *callbacks);
void print_callbacks(jvmtiEventCallbacks *callbacks) {
    print_object((void *) callbacks, sizeof(jvmtiEventCallbacks));
}

void print_object(void *objPtr, size_t size) {
    int i = 0;

    int8_t *iPtr = (int8_t *) objPtr;

    for (i = 0; i < size; i++) {
        printf("%0x ", iPtr[i]);
    }

    printf("\n");
}
