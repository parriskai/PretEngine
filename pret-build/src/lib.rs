pub mod compiled;
pub mod error;
pub mod spec;

use cargo_metadata::MetadataCommand;

pub fn list(){
    let metadata = MetadataCommand::new().exec().unwrap();
    let rootpackage = metadata.root_package().unwrap();

    for package in &rootpackage.dependencies{
        println!("{:#?}", package);
    }
}
