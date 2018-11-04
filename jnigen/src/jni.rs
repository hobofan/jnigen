use jnigen_shared::helpers::{CodegenStructure, Method};
use jnigen_shared::helpers;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn};

use util::{attr_class, attr_package};

pub fn jni_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let structure = CodegenStructure::from_file();

    let mut processed_item = add_attributes(item.clone());
    processed_item = set_visibility(processed_item);
    processed_item = set_fn_name(attr.clone(), processed_item);

    if let Some(mut structure) = structure {
        add_fn_to_structure(attr, item, &mut structure);

        structure.to_file().unwrap();
        helpers::set_out_dir_hint();
    }

    processed_item
}

fn add_attributes(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let expanded = quote! {
        #[no_mangle]
        #[allow(non_snake_case)]
        #input
    };
    TokenStream::from(expanded)
}

fn set_visibility(item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemFn);

    let visibility = syn::parse_str::<syn::Visibility>("pub").unwrap();
    input.vis = visibility;

    let abi = Some(syn::parse_str::<syn::Abi>("extern \"system\"").unwrap());
    input.abi = abi;

    let expanded = quote! {
        #input
    };
    TokenStream::from(expanded)
}

/// Sets the function name to a JNI compatible name like `Java_some_pkg_HelloWorld_hello`.
///
/// A function name consists of several parts. `Java_some_pkg_HelloWorld_hello`:
///
///   - `Java_` prefix
///   - (Optional) `some_pkg_`: underscore version of the package name `some.pkg`
///   - `HelloWorld_`: name of the class the function belongs to
///   - `hello`: the original function name
fn set_fn_name(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_input = parse_macro_input!(attr as AttributeArgs);
    let mut input = parse_macro_input!(item as ItemFn);

    let package_name_part = attr_package(&attr_input, false);
    let class_part = attr_class(&attr_input);

    let mut fn_name = input.ident.to_string();
    let mut fn_name_prefix = "Java_".to_owned();
    if let Some(package_name) = package_name_part {
        fn_name_prefix = format!("{}{}_", fn_name_prefix, package_name);
    }
    fn_name = format!("{}{}_{}", fn_name_prefix, class_part, fn_name);

    input.ident = syn::Ident::new(&fn_name, input.ident.span());

    let expanded = quote! {
        #input
    };
    TokenStream::from(expanded)
}

fn add_fn_to_structure(
    attr: TokenStream,
    item: TokenStream,
    structure: &mut CodegenStructure,
) -> TokenStream {
    let attr_input = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as ItemFn);

    let package_name_part = attr_package(&attr_input, false);
    if package_name_part.is_none() {
        return TokenStream::new();
    }
    let package_name_part = package_name_part.unwrap();
    let original_package_name = str::replace(&package_name_part, "_", ".");
    let package = structure.package(&original_package_name);

    let class_part = attr_class(&attr_input);
    let class = package.class(&class_part);

    let is_static = match input.decl.inputs.iter().nth(1).map(input_type) {
        Some(input_type) => match &*input_type {
            "JClass" => true,
            "JObject" => false,
            _ => true,
        },
        _ => false,
    };

    let fn_name = input.ident.to_string();
    class.methods.push(Method {
        is_static,
        name: fn_name,
        return_type: "String".to_owned(), // TODO
        parameters: vec![("String".to_string(), "input".to_string())], // TODO
    });

    TokenStream::new()
}

fn input_type(input: &syn::FnArg) -> String {
    match input {
        syn::FnArg::Captured(captured) => {
            if let syn::Type::Path(ref path) = captured.ty {
                return path.path.segments.iter().last().unwrap().ident.to_string();
            }
            panic!("Unrecognized function parameter")
        }
        _ => panic!("Unrecognized function parameter"),
    }
}
