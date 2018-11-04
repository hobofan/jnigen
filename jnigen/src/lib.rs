extern crate jnigen_shared;
extern crate proc_macro;
extern crate quote;
extern crate syn;

mod derive_jni;
mod jni;
mod jni_raw_java;
mod util;

use proc_macro::TokenStream;

use derive_jni::derive_jni_impl;
use jni::jni_impl;
use jni_raw_java::jni_raw_java_impl;

#[proc_macro_derive(JNI, attributes(jni_class))]
pub fn derive_jni(item: TokenStream) -> TokenStream {
    derive_jni_impl(item)
}

#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn jni(attr: TokenStream, item: TokenStream) -> TokenStream {
    jni_impl(attr, item)
}

#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn jni_raw_java(attr: TokenStream, item: TokenStream) -> TokenStream {
    jni_raw_java_impl(attr, item)
}
