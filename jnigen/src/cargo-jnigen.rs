extern crate jnigen_shared;

use std::fmt::Write as WriteFmt;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::fs::File;
use jnigen_shared::helpers::{self, Class, CodegenStructure, Package};

fn main() {
    let structure = CodegenStructure::from_outdir_hint();

    for package in structure.packages.iter() {
        create_package_dir(&package.name);
        create_package_content(&structure.library_name, &package);
    }
}

fn package_dir(package: &str) -> PathBuf {
    let package_pathy = str::replace(package, ".", "/");
    let path = Path::new(&helpers::target_directory())
        .join("jnigen")
        .join(package_pathy);
    path
}

fn create_package_dir(package: &str) {
    let path = package_dir(package);
    std::fs::create_dir_all(path).unwrap();
}

fn create_package_content(lib_name: &str, package: &Package) {
    for class in package.classes.iter() {
        create_package_class_file(lib_name, &package.name, class);
    }
}

fn create_package_class_file(lib_name: &str, package: &str, class: &Class) {
    let class_name = &class.name;
    let methods = &class.methods;

    let methods_part = {
        let mut lines = Vec::new();
        for method in methods {
            let mut parameter_parts = Vec::new();
            for parameter in &method.parameters {
                parameter_parts.push(format!("{} {}", parameter.0, parameter.1));
            }
            let static_part = match method.is_static {
                true => "static ",
                false => "",
            };

            let mut method_line = String::new();
            write!(
                &mut method_line,
                "public {3}native {0} {1}({2});",
                method.return_type,
                method.name,
                parameter_parts.join(",").to_owned(),
                static_part,
            ).unwrap();

            lines.push(method_line);
        }
        lines.join("\n")
    };

    let implements_part = {
        match class.implements.is_empty() {
            true => "".to_owned(),
            false => format!(" implements {}", class.implements.join(", ")),
        }
    };

    let file_path = package_dir(package).join(&format!("{}.java", class_name));
    let mut class_file = File::create(file_path).unwrap();
    write!(
        class_file,
        "
            package {0};

            public class {1}{4} {{
                {2}

                static {{
                    System.loadLibrary(\"{3}\");
                }}
            }}
        ",
        package, class_name, methods_part, lib_name, implements_part
    ).unwrap();
}
