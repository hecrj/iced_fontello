<div align="center">

# iced_fontello

[![Documentation](https://docs.rs/iced_fontello/badge.svg)](https://docs.rs/iced_fontello)
[![Crates.io](https://img.shields.io/crates/v/iced_fontello.svg)](https://crates.io/crates/iced_fontello)
[![License](https://img.shields.io/crates/l/iced_fontello.svg)](https://github.com/hecrj/iced_fontello/blob/master/LICENSE)
[![Downloads](https://img.shields.io/crates/d/iced_fontello.svg)](https://crates.io/crates/iced_fontello)
[![Test Status](https://img.shields.io/github/actions/workflow/status/hecrj/iced_fontello/test.yml?branch=master&event=push&label=test)](https://github.com/hecrj/iced_fontello/actions)
[![Discourse](https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fdiscourse.iced.rs%2Fsite%2Fstatistics.json&query=%24.users_count&suffix=%20users&label=discourse&color=5e7ce2)](https://discourse.iced.rs/)
[![Discord Server](https://img.shields.io/discord/628993209984614400?label=&labelColor=6A7EC2&logo=discord&logoColor=ffffff&color=7389D8)](https://discord.gg/3xZJ65GAhd)

A compile-time, type-safe icon font generator for [`iced`].
Powered by [Fontello].

[`iced`]: https://github.com/iced-rs/iced
[Fontello]: https://github.com/fontello/fontello

</div>

## Usage
Create a `.toml` file somewhere in your crate with the font definition:

```toml
# fonts/example-icons.toml
module = "icon"

[glyphs]
edit = "fontawesome-pencil"
save = "entypo-floppy"
trash = "typicons-trash"
```

The `module` value defines the Rust module that will be generated in your `src`
directory containing a type-safe API to use the font.

Each entry in the `[glyphs]` section corresponds to an icon. The keys will be
used as names for the functions of the module of the font; while the values
specify the glyph for that key using the format: `<font>-<glyph>`. You can browse
the available glyphs in [Fontello] or [the `fonts.json` file](fonts.json).

Next, add `iced_fontello` to your `build-dependencies`:

```rust
[build-dependencies]
iced_fontello = "0.13"
```

Then, call `iced_fontello::build` in your [build script](https://doc.rust-lang.org/cargo/reference/build-scripts.html),
passing the path of your font definition:

```rust
pub fn main() {
    println!("cargo::rerun-if-changed=fonts/example-icons.toml");
    iced_fontello::build("fonts/example-icons.toml").expect("Build example-icons font");
}
```

The library will generate the font and save its `.ttf` file right next to its definition.
In this example, the library would generate `fonts/example-icons.ttf`.

Finally, it will generate a type-safe `iced` API that lets you use the font. In our example:

```rust
// Generated automatically by iced_fontello at build time.
// Do not edit manually.
// d24460a00249b2acd0ccc64c3176452c546ad12d1038974e974d7bdb4cdb4a8f
use iced::widget::{text, Text};
use iced::Font;

pub const FONT: &[u8] = include_bytes!("../fonts/example-icons.ttf");

pub fn edit<'a>() -> Text<'a> {
    icon("\u{270E}")
}

pub fn save<'a>() -> Text<'a> {
    icon("\u{1F4BE}")
}

pub fn trash<'a>() -> Text<'a> {
    icon("\u{E10A}")
}

fn icon<'a>(codepoint: &'a str) -> Text<'a> {
    text(codepoint).font(Font::with_name("example-icons"))
}
```

Now you can simply add `mod icon;` to your `lib.rs` or `main.rs` file and enjoy your new font:

```rust
mod icon;

use iced::widget::row;

// ...

row![icon::edit(), icon::save(), icon::trash()].spacing(10)

// ...
```

Check out [the full example](example) to see it all in action.

## Packaging
If you plan to package your crate, you must make sure you include the generated module
and font file in the final package. `build` is effectively a no-op when the module and
the font already exist and are up-to-date.
