extern crate jni;
extern crate jnigen;

use jnigen::prelude::*;

use jni::JNIEnv;
use jni::objects::{JObject, JString};
use jni::sys::jstring;

#[jni(package = "some.pkg", class = "HelloWorld")]
fn helloInputConversion(env: JNIEnv, _obj: JObject, input: String) -> jstring {
    let output = env.new_string(format!("Hello there, {}!", input))
        .expect("Couldn't create java string");

    output.into_inner()
}

#[jni(package = "some.pkg", class = "HelloWorld")]
fn helloInputConversionParamName(env: JNIEnv, _obj: JObject, other_input_name: String) -> jstring {
    let output = env.new_string(format!("Hello there, {}!", other_input_name))
        .expect("Couldn't create java string");

    output.into_inner()
}

#[jni(package = "some.pkg", class = "HelloWorld")]
fn helloOutputConversion(env: JNIEnv, _obj: JObject, input: JString) -> String {
    let input: String = FromJNI::from_jni(&env, input);

    format!("Hello there, {}!", input)
}

#[jni(package = "some.pkg", class = "HelloWorld")]
fn helloBothConversion(env: JNIEnv, _obj: JObject, input: String) -> String {
    format!("Hello there, {}!", input)
}

#[jni(package = "some.pkg", class = "HelloWorld")]
fn helloInputConversionManual(env: JNIEnv, _obj: JObject, input: JString) -> jstring {
    let input: String = FromJNI::from_jni(&env, input);

    let output = env.new_string(format!("Hello there, {}!", input))
        .expect("Couldn't create java string");

    output.into_inner()
}

#[jni(package = "some.pkg", class = "HelloWorld")]
fn helloOutputConversionManual(env: JNIEnv, _obj: JObject, input: JString) -> jstring {
    let input: String = FromJNI::from_jni(&env, input);

    ToJNI::to_jni(format!("Hello there, {}!", input), &env)
}
