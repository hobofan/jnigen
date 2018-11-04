extern crate jni;
extern crate jnigen;

use jnigen::jni;

use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;

#[jni(package = "some.pkg", class = "HelloWorld")]
fn hello(env: JNIEnv, _class: JClass, input: JString) -> jstring {
    let input: String = env.get_string(input)
        .expect("Couldn't get java string!")
        .into();

    let output = env.new_string(format!("Hello there, {}!", input))
        .expect("Couldn't create java string");

    output.into_inner()
}
