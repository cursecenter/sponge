use rand::Rng;
use std::ops::Range;
use svg::node::element::{path::Data, Ellipse, Group, Path, Rectangle};
use svg::Document;

const COLOR_BLACK: &'static str = "#1F2434";
const COLOR_RED: &'static str = "#e1353d";
const COLOR_WHITE: &'static str = "#f9f9f9";
const COLOR_YELLOW: &'static str = "#f7f13d";

const EYE_C_RANGE: Range<i32> = -10..10;
const EYE_C_RANGE_SINGLE: Range<i32> = -100..100; // can go off screen
const EYE_SCALE_RANGE: Range<f32> = 0.8..1.2;

const TOOTH_LEFT_RANGE: Range<i32> = 5..30;
const TOOTH_RIGHT_RANGE: Range<i32> = -30..5;

// fn main() {
//     let doc = generate(new_base());
//     svg::save("test.svg", &doc).unwrap();
// }

pub fn generate() -> String {
    let mut rng = rand::thread_rng();

    let doc = new_base();
    let doc = generate_teeth(doc, &mut rng);
    let doc = generate_eyes(doc, &mut rng);

    let mut out = Vec::new();
    svg::write(&mut out, &doc).unwrap();
    String::from_utf8(out).unwrap()
}

fn generate_eyes(doc: Document, rng: &mut impl Rng) -> Document {
    let has_one_eye = rng.gen_bool(0.25);

    if has_one_eye {
        doc.add(
            Ellipse::new()
                .set("stroke-width", 0)
                .set("fill", COLOR_BLACK)
                .set("rx", 25)
                .set("ry", 25)
                .set("cx", 125)
                .set("cy", 150)
                .set(
                    "transform",
                    format!(
                        "translate({}, {}), scale({})",
                        rng.gen_range(EYE_C_RANGE_SINGLE),
                        rng.gen_range(EYE_C_RANGE_SINGLE),
                        rng.gen_range(EYE_SCALE_RANGE)
                    ),
                ),
        )
    } else {
        doc.add(
            Ellipse::new()
                .set("stroke-width", 0)
                .set("fill", COLOR_BLACK)
                .set("rx", 25)
                .set("ry", 25)
                .set("cx", 85)
                .set("cy", 75)
                .set(
                    "transform",
                    format!(
                        "translate({}, {}), scale({})",
                        rng.gen_range(EYE_C_RANGE),
                        rng.gen_range(EYE_C_RANGE),
                        rng.gen_range(EYE_SCALE_RANGE)
                    ),
                ),
        )
        .add(
            Ellipse::new()
                .set("stroke-width", 0)
                .set("fill", COLOR_BLACK)
                .set("rx", 25)
                .set("ry", 25)
                .set("cx", 165)
                .set("cy", 75)
                .set(
                    "transform",
                    format!(
                        "translate({}, {}), scale({})",
                        rng.gen_range(EYE_C_RANGE),
                        rng.gen_range(EYE_C_RANGE),
                        rng.gen_range(EYE_SCALE_RANGE)
                    ),
                ),
        )
    }
}

fn generate_teeth(doc: Document, rng: &mut impl Rng) -> Document {
    doc.add(
        Rectangle::new()
            .set("x", 85)
            .set("y", 160)
            .set("width", 30)
            .set("height", 40)
            .set("stroke-width", 0)
            .set("fill", COLOR_WHITE)
            .set(
                "transform",
                format!("rotate({}, 100, 160)", rng.gen_range(TOOTH_LEFT_RANGE)),
            ),
    )
    .add(
        Rectangle::new()
            .set("x", 140)
            .set("y", 160)
            .set("width", 30)
            .set("height", 40)
            .set("stroke-width", 0)
            .set("fill", COLOR_WHITE)
            .set(
                "transform",
                format!("rotate({}, 155, 160)", rng.gen_range(TOOTH_RIGHT_RANGE)),
            ),
    )
}

fn new_base() -> Document {
    let base_body = Rectangle::new()
        .set("width", 250)
        .set("height", 275)
        .set("stroke-width", 0)
        .set("fill", COLOR_YELLOW);

    let base_shirt = Rectangle::new()
        .set("y", 275)
        .set("width", 250)
        .set("height", 50)
        .set("stroke-width", 0)
        .set("fill", COLOR_WHITE);

    let base_pants = Rectangle::new()
        .set("y", 325)
        .set("width", 250)
        .set("height", 50)
        .set("stroke-width", 0)
        .set("fill", COLOR_RED);

    let base_tie = Path::new()
        .set("stroke-width", 0)
        .set("fill", COLOR_RED)
        .set(
            "d",
            Data::new()
                .move_to((115, 275))
                .line_by((10, 25))
                .line_by((-10, 25))
                .line_by((20, 0))
                .line_by((-10, -25))
                .line_by((10, -25))
                .line_by((-20, 0)),
        );

    let base_eyes = Group::new()
        .add(
            Ellipse::new()
                .set("stroke-width", 0)
                .set("fill", COLOR_WHITE)
                .set("rx", 50)
                .set("ry", 40)
                .set("cx", 85)
                .set("cy", 75),
        )
        .add(
            Ellipse::new()
                .set("stroke-width", 0)
                .set("fill", COLOR_WHITE)
                .set("rx", 50)
                .set("ry", 40)
                .set("cx", 165)
                .set("cy", 75),
        );

    let base_mouth = Path::new()
        .set("stroke-width", 0)
        .set("fill", COLOR_RED)
        // .set("transform", "rotate(-90)")
        .set(
            "d",
            Data::new()
                .move_to((35, 130))
                .cubic_curve_by((
                    0,   // x1
                    50,  // y1
                    40,  // x2
                    100, // y2
                    90,  // x
                    100, // y
                ))
                .cubic_curve_by((
                    50,   // x1
                    0,    // y1
                    90,   // x2
                    -50,  // y2
                    90,   // x
                    -100, // y
                ))
                .cubic_curve_by((
                    -10, // x1
                    -10, // y1
                    -40, // x2
                    30,  // y2
                    -90, // x
                    30,  // y
                ))
                .cubic_curve_by((
                    -50, // x1
                    0,   // y1
                    -80, // x2
                    -40, // y2
                    -90, // x
                    -30, // y
                ))
                .close(),
        );

    let base = Group::new()
        .add(base_body)
        .add(base_shirt)
        .add(base_pants)
        .add(base_tie)
        .add(base_eyes)
        .add(base_mouth);

    Document::new()
        .set("width", 250)
        .set("height", 375)
        .add(base)
}
