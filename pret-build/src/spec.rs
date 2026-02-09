use std::{collections::{HashMap, HashSet}, ffi::OsStr, fs, path::Path};

use serde::Deserialize;
use toml::{self, Deserializer};
use cargo_metadata::{Dependency, MetadataCommand};

use crate::error::{Error, TResult};

#[derive(Debug, Deserialize)]
pub struct RawPretFile {
    #[serde(default)]
    pub include: Vec<String>,

    #[serde(default)]
    pub entities: Vec<EntityDef>,

    #[serde(default)]
    pub components: Vec<ComponentDef>,

    #[serde(default)]
    pub handles: Vec<HandleDef>,

    #[serde(default)]
    pub worlds: Vec<WorldDef>,
}

#[derive(Debug, Deserialize)]
pub struct EntityDef {
    pub name: String,

    #[serde(default)]
    pub components: Vec<String>,

    #[serde(default)]
    pub include_in: Vec<String>,

    #[serde(default)]
    pub exclude_from: Vec<String>
}

#[derive(Debug, Deserialize)]
pub struct ComponentDef {
    pub name: String,

    #[serde(default)]
    pub handles: Vec<HandleRef>,

    #[serde(default)]
    #[serde(rename="with")]
    pub dependencies: Vec<String>,

    #[serde(default)]
    pub include_in: Vec<String>,

    #[serde(default)]
    pub exclude_from: Vec<String>
}

#[derive(Debug, Deserialize)]
pub struct HandleRef {
    #[serde(alias="when")]
    pub every: String,

    pub callback: String,

    #[serde(default)]
    pub with: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct HandleDef{
    pub name: String,

    #[serde(default)]
    pub context: Vec<String>,
    
    #[serde(default)]
    pub include_in: Vec<String>,

    #[serde(default)]
    pub exclude_from: Vec<String>
}

#[derive(Debug, Deserialize)]
pub struct WorldDef{
    pub name: String,

    #[serde(alias="exclude")]
    pub include: OptInOut
}

#[derive(Debug, Deserialize)]
pub enum OptInOut {
    #[serde(alias="optin")]
    #[serde(alias="opt_in")]
    OptIn,
    #[serde(alias="optout")]
    #[serde(alias="opt_out")]
    OptOut
}


pub struct PretSpec{
    loaded: HashSet<Box<OsStr>>
    
} impl PretSpec {
    pub fn new() -> PretSpec{
        PretSpec { loaded: HashSet::new() }
    }
    pub fn load_compiled(&mut self, path: &Path){
        if self.loaded.contains(path.as_os_str()){
            return; // Already loaded
        }
    }
    pub fn load(path: &Path) -> TResult<()>{
        let spec = fs::read_to_string(path.join("pret.toml")).map_err(|e|Error::IoError(e))?;

        let de = Deserializer::parse(spec.as_str()).map_err(|e| Error::DeserializationError(e))?;

        let pf = RawPretFile::deserialize(de).map_err(|e| Error::DeserializationError(e))?;

        let metadata = MetadataCommand::new().manifest_path(path.join("Cargo.toml")).exec().map_err(|e| Error::ManifestError(e))?;
        
        let package = (
            if let Some(root) = metadata.root_package(){Ok(root)}
            else {Err(Error::ManifestError(cargo_metadata::Error::CargoMetadata { stderr: "No Root package".to_string()}))}
        )?;

        let namespace: HashMap<String, Dependency> = package.dependencies.iter().map(|d| (d.name.clone(), d.clone())).collect();

        let mut includes: HashSet<Dependency> = HashSet::with_capacity(pf.include.len());

        for x in pf.include{
            if let Some(d) = namespace.get(&x){
                if !includes.insert(d.clone()){
                    println!("cargo::warning={x} is included twice");
                }
            } else{
                return Err(Error::DependencyError(format!("Dependency {x} not in the crate namespace")));
            }
        }
        Ok(())
    }
}