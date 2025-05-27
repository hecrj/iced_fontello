fn main() {
    println!("cargo::rerun-if-changed=fonts/fn-icons.toml");
    println!("cargo::rerun-if-changed=fonts/enum-icons.toml");
    
    iced_fontello::build("fonts/fn-icons.toml").expect("Generate fn-icons font");
    iced_fontello::build("fonts/enum-icons.toml").expect("Generate enum-icons font");
}
