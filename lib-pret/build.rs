use pret_build::spec::PretSpec;

fn main(){
    let pret = PretSpec::load(std::env::current_dir().unwrap().as_path()).unwrap();
}