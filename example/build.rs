fn main() {
    println!("cargo::rerun-if-changed=fonts/example-icons.toml");
    println!("cargo::rerun-if-changed=fonts/*.svg");
    iced_fontello::build("fonts/example-icons.toml").expect("Generate example-icons font");
}
