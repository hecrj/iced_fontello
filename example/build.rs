fn main() {
    println!("cargo::rerun-if-changed=fonts/example-icons.toml");
    println!("cargo::rerun-if-changed=fonts/iced.svg");
    println!("cargo::rerun-if-changed=fonts/rust.svg");
    iced_fontello::build("fonts/example-icons.toml").expect("Generate example-icons font");
}
