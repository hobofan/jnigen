use std::env;
use std::io;
use std::process;
use std::io::{Read, Seek, Write};
use std::path::{Path, PathBuf};
use std::fs::File;
use serde_derive::{Deserialize, Serialize};
use file_lock::FileLock;

pub fn library_name() -> String {
    let mut args = std::env::args().skip_while(|val| !val.starts_with("--manifest-path"));

    let manifest_path = match args.next() {
        Some(ref p) if p == "--manifest-path" => args.next(),
        Some(p) => Some(p.trim_left_matches("--manifest-path=").to_string()),
        None => None,
    };

    let metadata = cargo_metadata::metadata(manifest_path.as_ref().map(Path::new)).unwrap();
    let pkg_name = std::env::var("CARGO_PKG_NAME").unwrap();
    let library_package = metadata
        .packages
        .iter()
        .find(|n| n.name == pkg_name)
        .unwrap();
    let library_target = library_package
        .targets
        .iter()
        .find(|n| n.crate_types.contains(&"cdylib".to_owned()))
        .unwrap();

    return library_target.name.clone();
}

fn codegen_mode() -> bool {
    env::var("OUT_DIR").is_ok()
}

pub fn structurefile_path() -> PathBuf {
    Path::new(&env::var("OUT_DIR").expect("Can't get OUT_DIR envvar.")).join("jni-structure.json")
}

pub fn structurefile() -> io::Result<FileLock> {
    let path = structurefile_path();
    FileLock::lock(path.to_str().unwrap(), true, false)
}

pub fn structurefile_write() -> io::Result<FileLock> {
    let path = structurefile_path();
    FileLock::lock(path.to_str().unwrap(), true, true)
}

pub fn set_out_dir_hint() {
    if !codegen_mode() {
        return;
    }

    let out_dir = env::var("OUT_DIR").expect("Can't get OUT_DIR envvar.");
    let mut path = Path::new(&out_dir);
    while path.file_name().and_then(|n| n.to_str()) != Some("target") {
        if path.parent().is_none() {
            return;
        }
        path = path.parent().unwrap();
    }
    let path = path.join("jnigen-outdir-hint");

    let mut file = File::create(path).unwrap();
    file.write_all(out_dir.as_bytes()).unwrap();
}

pub fn target_directory() -> String {
    let mut args = std::env::args().skip_while(|val| !val.starts_with("--manifest-path"));

    let manifest_path = match args.next() {
        Some(ref p) if p == "--manifest-path" => args.next(),
        Some(p) => Some(p.trim_left_matches("--manifest-path=").to_string()),
        None => None,
    };

    let metadata = cargo_metadata::metadata(manifest_path.as_ref().map(Path::new)).unwrap();
    metadata.target_directory
}

pub fn get_out_dir_hint() -> Option<String> {
    let target_dir = target_directory();

    let hint_file_path = Path::new(&target_dir).join("jnigen-outdir-hint");
    File::open(hint_file_path)
        .and_then(|mut file| {
            let mut outdir = String::new();
            file.read_to_string(&mut outdir).unwrap();

            Ok(outdir)
        })
        .ok()
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CodegenStructure {
    #[serde(default)]
    pub pid: u32,
    #[serde(default)]
    pub library_name: String,
    #[serde(default)]
    pub packages: Vec<Package>,

    #[serde(skip)]
    #[serde(default)]
    file_lock: Option<FileLock>,
}

impl CodegenStructure {
    pub fn from_file() -> Option<Self> {
        if !codegen_mode() {
            return None;
        }
        let mut structure: (Self, Option<FileLock>) = {
            match structurefile() {
                Ok(mut file) => {
                    let mut file_contents = String::new();
                    file.file
                        .read_to_string(&mut file_contents)
                        .expect("Unable to read structurefile");

                    if file_contents.is_empty() {
                        file_contents = "{}".to_string();
                    }
                    (
                        serde_json::from_str(&file_contents).expect("Couldn't read file"),
                        Some(file),
                    )
                }
                Err(err) => {
                    if err.kind() == io::ErrorKind::NotFound {
                        (serde_json::from_str("{}").unwrap(), None)
                    } else {
                        panic!("Unable to open structurefile");
                    }
                }
            }
        };
        if structure.0.pid != process::id() {
            structure.0 = serde_json::from_str("{}").unwrap();
            structure.0.pid = process::id();
            structure.0.library_name = library_name();
        }
        structure.0.file_lock = structure.1;
        Some(structure.0)
    }

    pub fn from_outdir_hint() -> Self {
        let outdir = get_out_dir_hint().unwrap();
        let path = Path::new(&outdir).join("jni-structure.json");
        let mut file = File::open(path).expect("Unable to open structurefile");
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents).unwrap();

        if file_contents.is_empty() {
            file_contents = "{}".to_string();
        }
        serde_json::from_str(&file_contents).expect("Couldn't read file")
    }

    pub fn to_file(self) -> Result<(), serde_json::Error> {
        let mut file = structurefile_write().expect("Error aquiring write lock");
        // We need to truncate manually, as file_lock doesn't allow us to do that.
        file.file.set_len(0).unwrap();
        file.file.seek(std::io::SeekFrom::Start(0)).unwrap();
        serde_json::to_writer(&mut file.file, &self)?;
        Ok(file.file.flush().unwrap())
    }

    pub fn package(&mut self, package: &str) -> &mut Package {
        if self.packages.iter().find(|n| n.name == package).is_none() {
            self.packages.push(Package::new(package.to_owned()));
        }
        self.packages
            .iter_mut()
            .find(|n| n.name == package)
            .unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Package {
    pub name: String,
    pub classes: Vec<Class>,
}

impl Package {
    pub fn new(name: String) -> Self {
        Self {
            name,
            classes: Vec::new(),
        }
    }

    pub fn class(&mut self, class: &str) -> &mut Class {
        if self.classes.iter().find(|n| n.name == class).is_none() {
            self.classes.push(Class::new(class.to_owned()));
        }
        self.classes.iter_mut().find(|n| n.name == class).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Class {
    pub name: String,
    pub methods: Vec<Method>,
    pub implements: Vec<String>,
    pub raw_java: Vec<String>,
}

impl Class {
    pub fn new(name: String) -> Self {
        Self {
            name,
            methods: Vec::new(),
            implements: Vec::new(),
            raw_java: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Method {
    pub name: String,
    pub return_type: String,
    pub parameters: Vec<(String, String)>,
    pub is_static: bool,
}
