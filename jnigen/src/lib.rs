extern crate jnigen_shared;
extern crate proc_macro;
extern crate quote;
extern crate syn;

use jnigen_shared::helpers::{CodegenStructure, Method};
use jnigen_shared::helpers;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn, ItemStruct};

#[allow(non_snake_case)]
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

#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn jni_raw_java(attr: TokenStream, item: TokenStream) -> TokenStream {
    let structure = CodegenStructure::from_file();

    let attrs = parse_macro_input!(attr as AttributeArgs);
    let mut input = parse_macro_input!(item as ItemFn);

    let package = attr_package(&attrs, true).expect("\"package\" attribute value missing");
    let class_name = fn_class_part(&attrs);

    let raw_java = raw_java(&input.block.stmts);

    if let Some(mut structure) = structure {
        structure
            .package(&package)
            .class(&class_name)
            .raw_java
            .push(raw_java);

        structure.to_file().unwrap();
        helpers::set_out_dir_hint();
    }

    input.block.stmts = Vec::new();
    let expanded = quote! {
        #input
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(JNI, attributes(jni_class))]
pub fn derive_jni(item: TokenStream) -> TokenStream {
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

/// Transform derive attribute args to be the same format as proce_macro_attribute args
fn transform_derive_attrs(input_attrs: &[syn::Attribute]) -> AttributeArgs {
    let mut attrs: AttributeArgs = input_attrs
        .iter()
        .filter_map(|n| n.parse_meta().ok().map(|n| n.into()))
        .collect();
    attrs = attrs
        .into_iter()
        .filter_map(|n| {
            if let syn::NestedMeta::Meta(meta) = n {
                if let syn::Meta::List(list) = meta {
                    return Some(list.nested.into_iter().collect::<AttributeArgs>());
                }
            }
            None
        })
        .flatten()
        .collect::<AttributeArgs>();

    attrs
}

fn raw_java(stmts: &[syn::Stmt]) -> String {
    let first_stmt = &stmts[0];
    if let syn::Stmt::Expr(syn::Expr::Lit(expr_lit)) = first_stmt {
        if let syn::Lit::Str(ref lit_str) = expr_lit.lit {
            return lit_str.value();
        }
    }
    panic!("Unable to parse raw java block")
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

    let package_name_part = attr_package(&attr_input, false);
    if package_name_part.is_none() {
        return TokenStream::new();
    }
    let package_name_part = package_name_part.unwrap();
    let original_package_name = str::replace(&package_name_part, "_", ".");
    let package = structure.package(&original_package_name);

    let class_part = fn_class_part(&attr_input);
    let class = package.class(&class_part);

    let is_static = match input.decl.inputs.iter().nth(1).map(input_ident) {
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

fn input_ident(input: &syn::FnArg) -> String {
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

fn attr_package(attr_input: &syn::AttributeArgs, dotted: bool) -> Option<String> {
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
            if !dotted {
                package_name = str::replace(&package_name, ".", "_");
            }
            package_name_part = Some(package_name);
        }
    }

    package_name_part
}

fn attr_implements(attr_input: &syn::AttributeArgs) -> Vec<String> {
    let mut implements = Vec::new();
    for attribute in attr_input {
        let mut implements_opt: Option<String> = None;
        let implements_attr = match attribute {
            syn::NestedMeta::Meta(meta) => {
                if meta.name() == "implements" {
                    Some(attribute)
                } else {
                    None
                }
            }
            _ => None,
        };
        if let Some(syn::NestedMeta::Meta(syn::Meta::NameValue(val))) = implements_attr {
            if let syn::Lit::Str(ref lit_val) = val.lit {
                let mut implements_name = lit_val.value();
                implements_opt = Some(implements_name);
            }
        }
        if let Some(item) = implements_opt {
            implements.push(item);
        }
    }
    implements
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
