use jnigen_shared::helpers::{CodegenStructure, Method};
use jnigen_shared::helpers;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, AttributeArgs, ItemFn, Token};

use util::{attr_class, attr_package};

pub fn jni_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let structure = CodegenStructure::from_file();

    let mut processed_item = add_attributes(item.clone());
    processed_item = set_visibility(processed_item);
    processed_item = set_fn_name(attr.clone(), processed_item);
    processed_item = adjust_parameters(attr.clone(), processed_item);

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

fn adjust_parameters(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemFn);

    let new_inputs: syn::punctuated::Punctuated<syn::FnArg, Token!(,)> = input
        .decl
        .inputs
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, fn_arg)| {
            if i < 2 {
                return fn_arg;
            }
            let resolved_type = input_type_resolved(&input_ident(&fn_arg), &fn_arg);

            if let Some(stmt) = resolved_type.input_conversion_stmt {
                let mut new_stmts = vec![stmt];
                new_stmts.append(&mut input.block.stmts.clone());
                input.block.stmts = new_stmts;
            }

            if let syn::FnArg::Captured(captured) = fn_arg {
                let ty = syn::parse_str(&resolved_type.rust_result_type).unwrap();
                return syn::FnArg::Captured(syn::ArgCaptured {
                    pat: captured.pat,
                    colon_token: captured.colon_token,
                    ty,
                });
            }
            panic!()
        })
        .collect();
    input.decl.inputs = new_inputs;

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

    let parameters: Vec<_> = input
        .decl
        .inputs
        .iter()
        .skip(2)
        .map(|input| input_type_resolved(&input_ident(input), input))
        .collect();
    let java_parameters: Vec<(String, String)> = parameters
        .into_iter()
        .map(|n_type| (n_type.java_type, n_type.ident.unwrap()))
        .collect();

    let return_type = return_type_resolved(&input.decl.output);

    let fn_name = input.ident.to_string();
    class.methods.push(Method {
        is_static,
        name: fn_name,
        return_type,
        parameters: java_parameters,
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

fn input_type_resolved(ident: &str, input: &syn::FnArg) -> ResolvedType {
    let raw_type = input_type(input);
    resolve_type(Some(ident), &raw_type)
}

fn input_ident(input: &syn::FnArg) -> String {
    match input {
        syn::FnArg::Captured(captured) => {
            if let syn::Pat::Ident(ref ident) = captured.pat {
                return ident.ident.to_string();
            }
            panic!("Unrecognized function parameter ident")
        }
        _ => panic!("Unrecognized function parameter ident"),
    }
}

fn return_type(output: &syn::ReturnType) -> String {
    match output {
        syn::ReturnType::Type(_, inner_type) => {
            if let syn::Type::Path(ref path) = inner_type.as_ref() {
                return path.path.segments.iter().last().unwrap().ident.to_string();
            }
            panic!("Unable to process return type");
        }
        syn::ReturnType::Default => return "void".to_string(),
    }
}

fn return_type_resolved(output: &syn::ReturnType) -> String {
    let raw_type = return_type(output);
    resolve_type(None, &raw_type).java_type
}

#[derive(Debug)]
struct ResolvedType {
    rust_original_type: String,
    rust_result_type: String,
    java_type: String,
    input_conversion_stmt: Option<syn::Stmt>,
    ident: Option<String>,
}

impl ResolvedType {
    fn try_resolve_primitive(ident: Option<&str>, raw_type: &str) -> Option<Self> {
        let resolved = match raw_type {
            "jbyte" | "i8" => Some("byte"),
            "jchar" | "u16" => Some("char"),
            "jshort" | "i16" => Some("short"),
            "jint" | "i32" => Some("int"),
            "jlong" | "i64" => Some("long"),
            "jboolean" | "u8" => Some("boolean"),
            "jfloat" | "f32" => Some("float"),
            "jdouble" | "f64" => Some("double"),
            _ => None,
        };

        resolved.map(|java_type| Self::primitive(ident, raw_type, java_type))
    }

    fn primitive(ident: Option<&str>, rust_type: &str, java_type: &str) -> Self {
        Self {
            rust_original_type: rust_type.to_owned(),
            rust_result_type: rust_type.to_owned(),
            java_type: java_type.to_owned(),
            ident: ident.map(|n| n.to_owned()),
            ..ResolvedType::default()
        }
    }

    fn input_conversion(
        ident: &str,
        rust_original_type: &str,
        rust_result_type: &str,
        java_type: &str,
    ) -> Self {
        // let ident_rs: syn::Ident = parse_quote!(#ident);
        let ident_rs = syn::Ident::new(ident, proc_macro2::Span::call_site());
        // let rust_original_type_rs: syn::Type = parse_quote!(#rust_original_type);

        let stmt: syn::Stmt = parse_quote!{
            let #ident_rs : String = FromJNI::from_jni(&env, #ident_rs);
        };

        Self {
            rust_original_type: rust_original_type.to_owned(),
            rust_result_type: rust_result_type.to_owned(),
            java_type: java_type.to_owned(),
            input_conversion_stmt: Some(stmt),
            ident: Some(ident.to_owned()),
        }
    }
}

fn resolve_type(ident: Option<&str>, raw_type: &str) -> ResolvedType {
    if let Some(resolved) = ResolvedType::try_resolve_primitive(ident, raw_type) {
        return resolved;
    }
    match raw_type.as_ref() {
        "JString" | "jstring" => ResolvedType {
            rust_original_type: raw_type.to_owned(),
            rust_result_type: raw_type.to_owned(),
            java_type: "String".to_owned(),
            input_conversion_stmt: None,
            ident: ident.map(|n| n.to_owned()),
            ..ResolvedType::default()
        },
        "void" => ResolvedType {
            rust_original_type: raw_type.to_owned(),
            rust_result_type: raw_type.to_owned(),
            java_type: "void".to_owned(),
            ..ResolvedType::default()
        },
        "String" => ResolvedType::input_conversion(
            ident.expect("Expected ident for input conversion."),
            raw_type,
            "JString",
            "String",
        ),
        other => panic!("Unable to resolve type \"{}\"", other),
    }
}

impl Default for ResolvedType {
    fn default() -> Self {
        Self {
            rust_original_type: String::new(),
            rust_result_type: String::new(),
            java_type: String::new(),
            input_conversion_stmt: None,
            ident: None,
        }
    }
}
