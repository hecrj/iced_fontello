use std::path::Path;

use crate::{ChosenGlyph, Id, SvgIcon};

use usvg::{
    tiny_skia_path::{self, PathBuilder, PathSegment, Transform},
    Node, Options, Tree,
};

pub fn parse_image(icon_path: &Path, name: String, code: u64, uid: String) -> Option<ChosenGlyph> {
    let mut svg_path = String::new();

    let svg_str = std::fs::read_to_string(icon_path).unwrap_or_else(|error| {
        panic!(
            "SVG icon file \"{}\" could not be read: {error}",
            icon_path.display()
        )
    });

    let usvg_tree = Tree::from_str(&svg_str, &Options::default()).unwrap_or_else(|error| {
        panic!(
            "SVG icon file \"{}\" could not be parsed: {error}",
            icon_path.display()
        )
    });

    let mut svg_width = 0.0;

    // Create new `PathBuilder`
    let mut path_builder = PathBuilder::new();

    // Process the SVG tree using the path_builder
    usvg_tree
        .root()
        .children()
        .iter()
        .for_each(|n| process_node(n, &mut path_builder));

    // Finish the path builder and apply the final translate and scale transforms
    if let Some(final_path) = path_builder.finish() {
        let bounds = final_path.bounds();
        let scale = 1000.0 / bounds.height();
        let svg_x = bounds.left();
        let svg_y = bounds.top();

        // Translate and scale
        let final_path = final_path
            .transform(Transform::from_translate(-svg_x, -svg_y).post_scale(scale, scale))
            .unwrap_or_else(|| {
                panic!(
                    "SVG icon file \"{}\" failed to apply final translate and scale transform",
                    icon_path.display()
                )
            });

        let final_bounds = final_path.bounds();
        svg_width = final_bounds.width();

        // Write path to string
        svg_path = write_path(final_path);
    }

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

/// Process the nodes of an SVG tree by pushing each new path to the `path_builder`. The paths are
/// transformed before being pushed using their absolute transform which includes the parents
/// transforms.
fn process_node(node: &Node, path_builder: &mut PathBuilder) {
    match node {
        Node::Group(group) => {
            group
                .children()
                .iter()
                .for_each(|n| process_node(n, path_builder));
        }
        Node::Path(path) => {
            let t = path.abs_transform();
            let path_data = path.data().clone();
            if let Some(transformed_path) = path_data.transform(t) {
                path_builder.push_path(&transformed_path);
            } else {
                path_builder.push_path(path.data());
            }
        }
        Node::Image(_image) => {}
        Node::Text(_text) => {}
    }
}

/// Prints the `path` into a string of segments that can be used by fontello's glyph path
fn write_path(path: tiny_skia_path::Path) -> String {
    let mut s = String::new();
    for segment in path.segments() {
        match segment {
            PathSegment::MoveTo(p) => s.push_str(&format!("M {} {} ", p.x, p.y)),
            PathSegment::LineTo(p) => s.push_str(&format!("L {} {} ", p.x, p.y)),
            PathSegment::QuadTo(p0, p1) => {
                s.push_str(&format!("Q {} {} {} {} ", p0.x, p0.y, p1.x, p1.y))
            }
            PathSegment::CubicTo(p0, p1, p2) => s.push_str(&format!(
                "C {} {} {} {} {} {} ",
                p0.x, p0.y, p1.x, p1.y, p2.x, p2.y
            )),
            PathSegment::Close => s.push_str("Z "),
        }
    }
    s.pop(); // remove last trailing space
    s
}
