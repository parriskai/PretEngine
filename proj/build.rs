use pret_build::spec::ECSSpec;

fn main(){
    let pret = ECSSpec::load_pret(include_str!("pret.toml")).unwrap();
    println!("{:#?}",pret);
    panic!()
}