use jnigen_shared::helpers::CodegenStructure;
use jnigen_shared::helpers;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn};

use util::{attr_class, attr_package};

pub fn jni_raw_java_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let structure = CodegenStructure::from_file();

    let attrs = parse_macro_input!(attr as AttributeArgs);
    let mut input = parse_macro_input!(item as ItemFn);

    let package = attr_package(&attrs, true).expect("\"package\" attribute value missing");
    let class_name = attr_class(&attrs);

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

fn raw_java(stmts: &[syn::Stmt]) -> String {
    let first_stmt = &stmts[0];
    if let syn::Stmt::Expr(syn::Expr::Lit(expr_lit)) = first_stmt {
        if let syn::Lit::Str(ref lit_str) = expr_lit.lit {
            return lit_str.value();
        }
    }
    panic!("Unable to parse raw java block")
}
