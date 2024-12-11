#![allow(clippy::needless_doctest_main)]
//! A compile-time, type-safe icon font generator for [`iced`].
//! Powered by [Fontello].
//!
//! [`iced`]: https://github.com/iced-rs/iced
//! [Fontello]: https://github.com/fontello/fontello
//!
//! # Usage
//! Create a `.toml` file somewhere in your crate with the font definition:
//!
//! ```toml
//! # fonts/example-icons.toml
//! module = "icon"
//!
//! [glyphs]
//! edit = "fontawesome-pencil"
//! save = "entypo-floppy"
//! trash = "typicons-trash"
//! ```
//!
//! The `module` value defines the Rust module that will be generated in your `src`
//! directory containing a type-safe API to use the font.
//!
//! Each entry in the `[glyphs]` section corresponds to an icon. The keys will be
//! used as names for the functions of the module of the font; while the values
//! specify the glyph for that key using the format: `<font>-<glyph>`. You can browse
//! the available glyphs in [Fontello] or [the `fonts.json` file](fonts.json).
//!
//! Next, add `iced_fontello` to your `build-dependencies`:
//!
//! ```toml
//! [build-dependencies]
//! iced_fontello = "0.13"
//! ```
//!
//! Then, call `iced_fontello::build` in your [build script](https://doc.rust-lang.org/cargo/reference/build-scripts.html),
//! passing the path of your font definition:
//!
//! ```rust,no_run
//! pub fn main() {
//!     println!("cargo::rerun-if-changed=fonts/example-icons.toml");
//!     iced_fontello::build("fonts/example-icons.toml").expect("Build example-icons font");
//! }
//! ```
//!
//! The library will generate the font and save its `.ttf` file right next to its definition.
//! In this example, the library would generate `fonts/example-icons.ttf`.
//!
//! Finally, it will generate a type-safe `iced` API that lets you use the font. In our example:
//!
//! ```rust,ignore
//! // Generated automatically by iced_fontello at build time.
//! // Do not edit manually.
//! // d24460a00249b2acd0ccc64c3176452c546ad12d1038974e974d7bdb4cdb4a8f
//! use iced::widget::{text, Text};
//! use iced::Font;
//!
//! pub const FONT: &[u8] = include_bytes!("../fonts/example-icons.ttf");
//!
//! pub fn edit<'a>() -> Text<'a> {
//!     icon("\u{270E}")
//! }
//!
//! pub fn save<'a>() -> Text<'a> {
//!     icon("\u{1F4BE}")
//! }
//!
//! pub fn trash<'a>() -> Text<'a> {
//!     icon("\u{E10A}")
//! }
//!
//! fn icon<'a>(codepoint: &'a str) -> Text<'a> {
//!     text(codepoint).font(Font::with_name("example-icons"))
//! }
//! ```
//!
//! Now you can simply add `mod icon;` to your `lib.rs` or `main.rs` file and enjoy your new font:
//!
//! ```rust,ignore
//! mod icon;
//!
//! use iced::widget::row;
//!
//! // ...
//!
//! row![icon::edit(), icon::save(), icon::trash()].spacing(10)
//!
//! // ...
//! ```
//!
//! Check out [the full example](example) to see it all in action.
//!
//! # Packaging
//! If you plan to package your crate, you must make sure you include the generated module
//! and font file in the final package. `build` is effectively a no-op when the module and
//! the font already exist and are up-to-date.
mod svg_parser;

use rand::Rng;
use reqwest::blocking as reqwest;
use serde::{Deserialize, Serialize};

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn build(path: impl AsRef<Path>) -> Result<(), Error> {
    let path = path.as_ref();

    let definition: Definition = {
        let contents = fs::read_to_string(path).unwrap_or_else(|error| {
            panic!(
                "Font definition {path} could not be read: {error}",
                path = path.display()
            )
        });

        toml::from_str(&contents).unwrap_or_else(|error| {
            panic!(
                "Font definition {path} is invalid: {error}",
                path = path.display()
            )
        })
    };

    let fonts = parse_fonts();

    let mut glyphs: BTreeMap<String, ChosenGlyph> = definition
        .glyphs
        .into_iter()
        .map(|(name, id)| {
            let Some((font_name, glyph)) = id.split_once('-') else {
                panic!(
                    "Invalid glyph identifier: \"{id}\"\n\
                    Glyph identifier must have \"<font>-<name>\" format"
                )
            };

            let Some(font) = fonts.get(font_name) else {
                panic!(
                    "Font \"{font_name}\" was not found. Available fonts are:\n{}",
                    fonts
                        .keys()
                        .map(|name| format!("- {name}"))
                        .collect::<Vec<_>>()
                        .join("\n")
                );
            };

            let Some(glyph) = font.glyphs.get(glyph) else {
                // TODO: Display similarly named candidates
                panic!(
                    "Glyph \"{glyph}\" was not found. Available glyphs are:\n{}",
                    font.glyphs
                        .keys()
                        .map(|name| format!("- {name}"))
                        .collect::<Vec<_>>()
                        .join("\n")
                );
            };

            (
                name,
                ChosenGlyph {
                    uid: glyph.uid.clone(),
                    css: glyph.name.clone(),
                    code: glyph.code,
                    src: font.name.clone(),
                    selected: None,
                    svg: None,
                },
            )
        })
        .collect();

    let svg_icons: BTreeMap<String, ChosenGlyph> =
        if let (Some(parent), Some(svg_icons)) = (path.parent(), definition.svg_icons) {
            let mut rng = rand::thread_rng();
            svg_icons
                .into_iter()
                .enumerate()
                .flat_map(|(i, (name, id))| {
                    if glyphs.contains_key(&name) {
                        panic!("name \"{name}\" for svg icon was already used on a glyph");
                    }
                    let uid: String = (0..32).fold(String::new(), |mut s, _| {
                        write!(&mut s, "{:x}", rng.gen_range(0..15)).unwrap_or_default();
                        s
                    });
                    let code = 0xe800 + (i as u64);
                    let id = id.trim_end_matches(".svg");
                    let icon_path = parent.join(id).with_extension("svg");

                    svg_parser::parse_image(&icon_path, name.clone(), code, uid)
                        .map(|glyph| (name, glyph))
                })
                .collect()
        } else {
            BTreeMap::new()
        };

    if !svg_icons.is_empty() {
        glyphs.extend(svg_icons);
    }

    let file_name = path
        .file_stem()
        .expect("Get file stem from definition path")
        .to_string_lossy()
        .into_owned();

    let config = Config {
        name: file_name.clone(),
        css_prefix_text: "icon-",
        css_use_suffix: false,
        hinting: true,
        units_per_em: 1000,
        ascent: 850,
        glyphs: glyphs.values().cloned().collect(),
    };

    let hash = {
        use sha2::Digest as _;

        let mut hasher = sha2::Sha256::new();
        hasher.update(serde_json::to_string(&config).expect("Serialize config as JSON"));

        format!("{:x}", hasher.finalize())
    };

    let module_target = PathBuf::new()
        .join("src")
        .join(definition.module.replace("::", "/"))
        .with_extension("rs");

    let module_contents = fs::read_to_string(&module_target).unwrap_or_default();
    let module_hash = module_contents
        .lines()
        .nth(2)
        .unwrap_or_default()
        .trim_start_matches("// ");

    if hash != module_hash || !path.with_extension("ttf").exists() {
        let client = reqwest::Client::new();
        let session = client
            .post("https://fontello.com/")
            .multipart(
                reqwest::multipart::Form::new().part(
                    "config",
                    reqwest::multipart::Part::text(
                        serde_json::to_string(&config).expect("Serialize Fontello config"),
                    )
                    .file_name("config.json"),
                ),
            )
            .send()
            .and_then(reqwest::Response::error_for_status)
            .and_then(reqwest::Response::text)
            .expect("Create Fontello session");

        let font = client
            .get(format!("https://fontello.com/{session}/get"))
            .send()
            .and_then(reqwest::Response::error_for_status)
            .and_then(reqwest::Response::bytes)
            .expect("Download Fontello font");

        let mut archive =
            zip::ZipArchive::new(io::Cursor::new(font)).expect("Parse compressed font");

        let mut font_file = (0..archive.len())
            .find(|i| {
                let file = archive.by_index(*i).expect("Access zip archive by index");

                file.name().ends_with(&format!("{file_name}.ttf"))
            })
            .and_then(|i| archive.by_index(i).ok())
            .expect("Find font file in zipped archive");

        io::copy(
            &mut font_file,
            &mut fs::File::create(path.with_extension("ttf")).expect("Create font file"),
        )
        .expect("Extract font file");
    }

    let relative_path = PathBuf::from("../".repeat(definition.module.split("::").count()));

    let mut module = String::new();

    module.push_str(&format!(
        "// Generated automatically by iced_fontello at build time.\n\
         // Do not edit manually. Source: {source}\n\
         // {hash}\n\
         use iced::widget::{{text, Text}};\n\
         use iced::Font;\n\n\
         pub const FONT: &[u8] = include_bytes!(\"{path}\");\n\n",
        source = relative_path.join(path.with_extension("toml")).display(),
        path = relative_path.join(path.with_extension("ttf")).display()
    ));

    for (name, glyph) in glyphs {
        module.push_str(&format!(
            "\
pub fn {name}<'a>() -> Text<'a> {{
    icon(\"\\u{{{code:X}}}\")
}}\n\n",
            code = glyph.code
        ));
    }

    module.push_str(&format!(
        "\
fn icon(codepoint: &str) -> Text<'_> {{
    text(codepoint).font(Font::with_name(\"{file_name}\"))
}}\n"
    ));

    if module != module_contents {
        if let Some(directory) = module_target.parent() {
            fs::create_dir_all(directory).expect("Create parent directory of font module");
        }

        fs::write(module_target, module).expect("Write font module");
    }

    Ok(())
}

#[derive(Debug, Clone)]
pub enum Error {}

#[derive(Debug, Clone, Deserialize)]
struct Definition {
    module: String,
    glyphs: BTreeMap<String, String>,
    svg_icons: Option<BTreeMap<String, String>>,
}

#[derive(Debug, Clone)]
struct Font {
    name: String,
    glyphs: BTreeMap<String, Glyph>,
}

#[derive(Debug, Clone, Deserialize)]
struct Glyph {
    uid: Id,
    code: u64,
    #[serde(rename = "css")]
    name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub(crate) struct Id(String);

#[derive(Serialize)]
struct Config {
    name: String,
    css_prefix_text: &'static str,
    css_use_suffix: bool,
    hinting: bool,
    units_per_em: u32,
    ascent: u32,
    glyphs: Vec<ChosenGlyph>,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct ChosenGlyph {
    uid: Id,
    css: String,
    code: u64,
    src: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    selected: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    svg: Option<SvgIcon>,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct SvgIcon {
    path: String,
    width: f64,
}

fn parse_fonts() -> BTreeMap<String, Font> {
    #[derive(Deserialize)]
    struct ItemSchema {
        font: FontSchema,
        glyphs: Vec<Glyph>,
    }

    #[derive(Deserialize)]
    struct FontSchema {
        fontname: String,
    }

    let items: Vec<ItemSchema> =
        serde_json::from_str(include_str!("../fonts.json")).expect("Deserialize fonts");

    items
        .into_iter()
        .map(|item| {
            (
                item.font.fontname.clone(),
                Font {
                    name: item.font.fontname,
                    glyphs: item
                        .glyphs
                        .into_iter()
                        .map(|glyph| (glyph.name.clone(), glyph))
                        .collect(),
                },
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_fonts() {
        assert!(!parse_fonts().is_empty());
    }
}
