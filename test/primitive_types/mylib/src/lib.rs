extern crate jni;
extern crate jnigen;

use jnigen::prelude::*;

use jni::JNIEnv;
use jni::objects::JObject;
use jni::sys::{jboolean, jbyte, jchar, jdouble, jfloat, jint, jlong, jshort};

#[jni(package = "some.pkg", class = "HelloWorld")]
fn plusOneByte(_env: JNIEnv, _obj: JObject, input: jbyte) -> jbyte {
    input + 1
}

#[jni(package = "some.pkg", class = "HelloWorld")]
fn plusOneChar(_env: JNIEnv, _obj: JObject, input: jchar) -> jchar {
    input + 1
}

#[jni(package = "some.pkg", class = "HelloWorld")]
fn plusOneInt(_env: JNIEnv, _obj: JObject, input: jint) -> jint {
    input + 1
}

#[jni(package = "some.pkg", class = "HelloWorld")]
fn plusOneShort(_env: JNIEnv, _obj: JObject, input: jshort) -> jshort {
    input + 1
}

#[jni(package = "some.pkg", class = "HelloWorld")]
fn plusOneLong(_env: JNIEnv, _obj: JObject, input: jlong) -> jlong {
    input + 1
}

#[jni(package = "some.pkg", class = "HelloWorld")]
fn flipBoolean(_env: JNIEnv, _obj: JObject, input: jboolean) -> jboolean {
    if input == 0 {
        return 1;
    } else {
        return 0;
    }
}

#[jni(package = "some.pkg", class = "HelloWorld")]
fn twiceFloat(_env: JNIEnv, _obj: JObject, input: jfloat) -> jfloat {
    input * 2f32
}

#[jni(package = "some.pkg", class = "HelloWorld")]
fn twiceDouble(_env: JNIEnv, _obj: JObject, input: jdouble) -> jdouble {
    input * 2f64
}

#[jni(package = "some.pkg", class = "HelloWorld")]
fn doVoid(_env: JNIEnv, _obj: JObject, input: jint) {
    let _new = input + input;
}
