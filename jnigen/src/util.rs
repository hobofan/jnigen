use syn;
use syn::AttributeArgs;

pub fn attr_implements(attr_input: &syn::AttributeArgs) -> Vec<String> {
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

pub fn attr_class(attr_input: &syn::AttributeArgs) -> String {
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

pub fn attr_package(attr_input: &syn::AttributeArgs, dotted: bool) -> Option<String> {
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

/// Transform derive attribute args to be the same format as proce_macro_attribute args
pub fn transform_derive_attrs(input_attrs: &[syn::Attribute]) -> AttributeArgs {
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
