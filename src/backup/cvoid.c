#include <jvmti.h>
#include <jni.h>
#include <string.h>

void C_dostuff(jvmtiEnv *jvmti);


void C_dostuff(jvmtiEnv *jvmti) {
    printf("C DOSTUFF\n");
}
