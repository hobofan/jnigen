extern crate jni;
extern crate jnigen;

use jnigen::prelude::*;

use jni::JNIEnv;
use jni::objects::{JObject, JString};
use jni::sys::jstring;

#[derive(JNI)]
#[jni_class(package = "some.pkg")]
#[jni_class(implements = "other.OtherInterface")]
pub struct HelloWorld;

#[jni(package = "some.pkg", class = "HelloWorld")]
fn hello(env: JNIEnv, _obj: JObject, input: JString) -> jstring {
    let input: String = env.get_string(input)
        .expect("Couldn't get java string!")
        .into();

    let output = env.new_string(format!("Hello there, {}!", input))
        .expect("Couldn't create java string");

    output.into_inner()
}
