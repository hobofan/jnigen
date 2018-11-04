use jnigen_shared::helpers::CodegenStructure;
use jnigen_shared::helpers;
use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemStruct};

use util::{attr_implements, attr_package, transform_derive_attrs};

pub fn derive_jni_impl(item: TokenStream) -> TokenStream {
    let structure = CodegenStructure::from_file();
    if let Some(mut structure) = structure {
        let input = parse_macro_input!(item as ItemStruct);
        let attrs = transform_derive_attrs(&input.attrs);

        let class_name = input.ident.to_string();
        let package = attr_package(&attrs, true).expect("\"package\" attribute value missing");
        let mut attr_implements: Vec<String> = attr_implements(&attrs);

        structure
            .package(&package)
            .class(&class_name)
            .implements
            .append(&mut attr_implements);

        structure.to_file().unwrap();
        helpers::set_out_dir_hint();
    }
    "".parse().unwrap()
}
