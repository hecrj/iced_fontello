use reqwest::blocking as reqwest;
use serde::{Deserialize, Serialize};

use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn build(path: impl AsRef<Path>) -> Result<(), Error> {
    let path = path.as_ref();

    println!("cargo::rerun-if-changed={path}.toml", path = path.display());

    let definition: Definition = {
        let contents =
            fs::read_to_string(path.with_extension("toml")).expect("Read font definition");

        toml::from_str(&contents).expect("Deserialize font definition")
    };

    let fonts = parse_fonts();

    let glyphs: BTreeMap<String, ChosenGlyph> = definition
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
                },
            )
        })
        .collect();

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

    #[derive(Clone, Serialize)]
    struct ChosenGlyph {
        uid: Id,
        css: String,
        code: u64,
        src: String,
    }

    let file_name = path
        .file_stem()
        .expect("Get file stem from definition path")
        .to_string_lossy()
        .into_owned();

    if !path.with_extension("ttf").exists() {
        let config = Config {
            name: file_name.clone(),
            css_prefix_text: "icon-",
            css_use_suffix: false,
            hinting: true,
            units_per_em: 1000,
            ascent: 850,
            glyphs: glyphs.values().cloned().collect(),
        };

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

    let mut module = String::new();

    module.push_str(&format!(
        "// Generated automatically by iced_fontello at build time.\n\
         // Do not edit manually.\n\
         use iced::widget::{{text, Text}};\n\
         use iced::Font;\n\n\
         pub const FONT: &[u8] = include_bytes!(\"../{path}.ttf\");\n\n",
        path = path.display()
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
fn icon<'a>(codepoint: &'a str) -> Text<'a> {{
    text(codepoint).font(Font::with_name(\"{file_name}\"))
}}\n"
    ));

    let module_target = PathBuf::new()
        .join("src")
        .join(definition.module)
        .with_extension("rs");

    if !module_target.exists()
        || module != fs::read_to_string(&module_target).expect("Read module contents")
    {
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
struct Id(String);

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
