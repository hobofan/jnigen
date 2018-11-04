extern crate jni;
extern crate jnigen_macro;

pub mod prelude {
    pub use jnigen_macro::{jni, jni_raw_java, JNI};
    pub use super::FromJNI;
}

use jni::JNIEnv;

pub trait FromJNI<'a> {
    type Input;

    fn from_jni(env: &JNIEnv, input: Self::Input) -> Self;
}

impl<'a> FromJNI<'a> for String {
    type Input = jni::objects::JString<'a>;

    fn from_jni(env: &JNIEnv, input: Self::Input) -> Self {
        env.get_string(input)
            .expect("Couldn't get java string!")
            .into()
    }
}
