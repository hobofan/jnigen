extern crate jni;
extern crate jnigen;

use jnigen::{jni, jni_raw_java, JNI};

use jni::JNIEnv;
use jni::objects::{JObject, JString};
use jni::sys::jstring;

#[derive(JNI)]
#[jni_class(package = "some.pkg")]
#[jni_class(implements = "other.OtherInterface")]
pub struct HelloWorld;

#[jni_raw_java(package = "some.pkg", class = "HelloWorld")]
fn hello() {
    r###"
        public String hello(String input) {
            return "This ignore the input value";
        }
    "###
}
