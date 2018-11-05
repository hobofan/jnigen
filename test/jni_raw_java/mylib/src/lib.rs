extern crate jni;
extern crate jnigen;

use jnigen::prelude::*;

#[derive(JNI)]
#[jni_class(package = "some.pkg")]
#[jni_class(implements = "other.OtherInterface")]
pub struct HelloWorld;

#[allow(dead_code)]
#[jni_raw_java(package = "some.pkg", class = "HelloWorld")]
fn hello() {
    r###"
        public String hello(String input) {
            return "This ignore the input value";
        }
    "###
}
