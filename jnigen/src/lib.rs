extern crate jnigen_shared;
extern crate proc_macro;
extern crate quote;
extern crate syn;

use jnigen_shared::helpers::{Class, CodegenStructure, Method, Package};
use jnigen_shared::helpers;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn};

#[proc_macro_attribute]
pub fn jni(attr: TokenStream, item: TokenStream) -> TokenStream {
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

    let package_name_part = fn_package_name_part(&attr_input);
    let class_part = fn_class_part(&attr_input);

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

    let package_name_part = fn_package_name_part(&attr_input);
    if package_name_part.is_none() {
        return TokenStream::new();
    }
    let package_name_part = package_name_part.unwrap();
    let original_package_name = str::replace(&package_name_part, "_", ".");
    let package: &mut Package = {
        if structure
            .packages
            .iter()
            .find(|n| n.name == original_package_name)
            .is_none()
        {
            structure
                .packages
                .push(Package::new(original_package_name.clone()));
        }
        structure
            .packages
            .iter_mut()
            .find(|n| n.name == original_package_name)
            .unwrap()
    };

    let class_part = fn_class_part(&attr_input);
    let class = {
        if package
            .classes
            .iter()
            .find(|n| n.name == class_part)
            .is_none()
        {
            package.classes.push(Class::new(class_part.clone()));
        }
        package
            .classes
            .iter_mut()
            .find(|n| n.name == class_part)
            .unwrap()
    };

    let fn_name = input.ident.to_string();
    class.methods.push(Method {
        name: fn_name,
        return_type: "String".to_owned(), // TODO
        parameters: vec![("String".to_string(), "input".to_string())],
    });

    TokenStream::new()
}

fn fn_package_name_part(attr_input: &syn::AttributeArgs) -> Option<String> {
    let mut package_name_part: Option<String> = None;
    let package_attr = attr_input.iter().find(|n| {
        if let syn::NestedMeta::Meta(meta) = n {
            if meta.name() == "package" {
                return true;
            }
        }
        return false;
    });
    if let Some(syn::NestedMeta::Meta(syn::Meta::NameValue(val))) = package_attr {
        if let syn::Lit::Str(ref lit_val) = val.lit {
            let mut package_name = lit_val.value();
            package_name = str::replace(&package_name, ".", "_");
            package_name_part = Some(package_name);
        }
    }

    package_name_part
}

fn fn_class_part(attr_input: &syn::AttributeArgs) -> String {
    let class_attr = attr_input.iter().find(|n| {
        if let syn::NestedMeta::Meta(meta) = n {
            if meta.name() == "class" {
                return true;
            }
        }
        return false;
    });
    if class_attr.is_none() {
        panic!("The \"class\" attribute has to be provided.");
    }
    let class_attr = class_attr.unwrap();
    if let syn::NestedMeta::Meta(syn::Meta::NameValue(val)) = class_attr {
        if let syn::Lit::Str(ref lit_val) = val.lit {
            return lit_val.value();
        }
    }
    unimplemented!()
}
