use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputFile{
    crates: Vec<Crate>,
    dependencies: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Crate{
    path: String,
    entities: Vec<Entity>,
    components: Vec<Entity>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entity{
    name: String,
    dependencies: Vec<(String, String)>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Component{
    name: String,
    dependencies: Vec<(String, String)>
}