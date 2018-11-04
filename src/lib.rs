extern crate jni;
extern crate jnigen_macro;

pub mod prelude {
    pub use jnigen_macro::{jni, jni_raw_java, JNI};
    pub use super::{FromJNI, ToJNI};
}

use jni::JNIEnv;

pub trait FromJNI<'a> {
    type Input;

    fn from_jni(env: &JNIEnv, input: Self::Input) -> Self;
}

pub trait ToJNI<'a> {
    type Output;

    fn to_jni(self, env: &JNIEnv) -> Self::Output;
}

impl<'a> FromJNI<'a> for String {
    type Input = jni::objects::JString<'a>;

    fn from_jni(env: &JNIEnv, input: Self::Input) -> Self {
        env.get_string(input)
            .expect("Couldn't get java string!")
            .into()
    }
}

impl<'a> ToJNI<'a> for String {
    type Output = jni::sys::jstring;

    fn to_jni(self, env: &JNIEnv) -> Self::Output {
        let output = env.new_string(self).expect("Couldn't create java string");
        output.into_inner()
    }
}
