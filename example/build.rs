fn main() {
    println!("cargo::rerun-if-changed=fonts/example-icons.toml");
    iced_fontello::build("fonts/example-icons.toml").expect("Generate example-icons font");
}
