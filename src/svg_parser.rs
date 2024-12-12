use std::{collections::BTreeMap, path::Path};

use svg::{
    node::element::tag::{self, Type},
    parser::Event,
};
use svg_path_ops::pt::PathTransformer;

use crate::{ChosenGlyph, Id, SvgIcon};

pub fn parse_image(icon_path: &Path, name: String, code: u64, uid: String) -> Option<ChosenGlyph> {
    let mut content = String::new();
    let parser = svg::open(icon_path, &mut content).unwrap_or_else(|error| {
        panic!(
            "SVG icon file \"{}\" could not be read: {error}",
            icon_path.display()
        )
    });
    let mut svg_path = String::new();
    let mut group_nest_level = 0;
    let mut svg_view_box = String::new();
    let mut svg_x = 0.0;
    let mut svg_y = 0.0;
    let mut svg_width = 0.0;
    let mut svg_height = 0.0;
    let mut transforms = String::new();
    let mut transforms_map = BTreeMap::new();

    for event in parser {
        match event {
            Event::Tag(tag::Group, Type::Start, attributes) => {
                group_nest_level += 1;
                if let Some(t) = attributes.get("transform") {
                    let t = t.to_string();
                    transforms = transforms + " " + &t;
                    transforms_map.insert(group_nest_level, t);
                }
            }
            Event::Tag(tag::Group, Type::End, _) => {
                if let Some(t) = transforms_map.remove(&group_nest_level) {
                    transforms = transforms.trim_end_matches(&t).to_string();
                }
                group_nest_level -= 1;
            }
            Event::Tag(tag::Path, _, attributes) => {
                if let Some(p) = attributes.get("d") {
                    let path_transforms = if let Some(t) = attributes.get("transform") {
                        transforms.clone() + " " + t.to_string().as_str()
                    } else {
                        transforms.clone()
                    };
                    let mut current_path = p.to_string();
                    if !transforms.is_empty() {
                        let mut pt = PathTransformer::new(current_path);
                        pt.transform(path_transforms);
                        current_path = pt.to_string();
                    }
                    svg_path += &current_path;
                }
            }
            Event::Tag(tag::SVG, _, attributes) => {
                if let Some(x) = attributes.get("x") {
                    svg_x = x
                        .to_string()
                        .parse()
                        .expect("couldn't parse \"x\" from svg icon");
                }
                if let Some(y) = attributes.get("y") {
                    svg_y = y
                        .to_string()
                        .parse()
                        .expect("couldn't parse \"y\" from svg icon");
                }
                if let Some(w) = attributes.get("width") {
                    let w = w.to_string();
                    if w.ends_with("%") {
                        // Ignore percentage sizes just like fontello does here:
                        // https://github.com/fontello/fontello/blob/master/client/fontello/app/import/_svg_image_flatten.js#L198-L203
                        svg_width = 0.0;
                    } else {
                        svg_width = w.parse().expect("couldn't parse \"width\" from svg icon");
                    }
                }
                if let Some(h) = attributes.get("height") {
                    let h = h.to_string();
                    if h.ends_with("%") {
                        // Ignore percentage sizes just like fontello does here:
                        // https://github.com/fontello/fontello/blob/master/client/fontello/app/import/_svg_image_flatten.js#L198-L203
                        svg_height = 0.0;
                    } else {
                        svg_height = h.parse().expect("couldn't parse \"height\" from svg icon");
                    }
                }
                if let Some(view_box) = attributes.get("viewBox") {
                    svg_view_box = view_box.to_string();
                }
            }
            _ => {}
        }
    }

    let mut pt = PathTransformer::new(svg_path);
    let mut actual_height = svg_height;

    if !svg_view_box.is_empty() {
        let vb = svg_view_box.split(" ").collect::<Vec<_>>();
        if vb.len() != 4 {
            panic!(
                "SVG Icon \"{}\" should have viewBox with 4 elements)",
                icon_path.display()
            );
        }
        let vb = vb
            .into_iter()
            .map(|e| e.parse().expect("viewBox element should be a number"))
            .collect::<Vec<f64>>();
        let _x = vb[0];
        let y = vb[1];
        let vb_width = vb[2];
        let vb_height = vb[3];
        if svg_width == 0.0 {
            svg_width = vb_width;
        }
        actual_height = f64::max(actual_height, vb_height - y);
    }

    // Translate the final path
    pt.translate(-svg_x, -svg_y);

    // Scale the final path
    let scale = 1000.0 / actual_height;
    pt.scale(scale, scale);
    svg_width *= scale;

    // Final transformed path
    svg_path = pt.to_string();

    if !svg_path.is_empty() && svg_width != 0.0 {
        Some(ChosenGlyph {
            uid: Id(uid),
            css: name,
            code,
            src: "custom_icons".into(),
            selected: Some(true),
            svg: Some(SvgIcon {
                path: svg_path,
                width: svg_width,
            }),
        })
    } else {
        None
    }
}
