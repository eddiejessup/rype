extern crate rype;

use std::fs::File;
use std::io::prelude::*;

use rype::doc::{Page, HBox, Glyph};

const R_C_BOX: f64 = 0.1;
const K_C_BOX: f64 = 0.1;
const K_LINE: f64 = 0.05;
const R_LINE: f64 = 0.4;

fn main() {
    let nx = 500;
    let ny = 500;

    let mut page = Page::new(nx, ny);

    let circ_glyph = Glyph::circle(0.02, nx, ny);

    let circ_glyph_name = String::from("circle");
    page.glyphs.insert(circ_glyph_name.clone(), circ_glyph);

    let mut h_box = HBox {
        c_boxes: vec![],
        base_line: R_LINE,
        base_line_k: K_LINE,
        c_box_sep: R_C_BOX,
        c_box_k: K_C_BOX,
    };

    h_box.add_c_box(circ_glyph_name.clone(), 0.11, 0.23);
    h_box.add_c_box(circ_glyph_name.clone(), 0.67, 0.6);
    h_box.add_c_box(circ_glyph_name.clone(), 0.31, 0.4);
    h_box.add_c_box(circ_glyph_name.clone(), 0.77, 0.8);

    page.h_boxes.push(h_box);

    for t in 0..10000 {
        page.iterate(0.01);
        if t % 50 == 0 {
            // println!("hihi");
            let mut f = File::create(format!("dat/out_{0:010}.csv", t)).expect("Could not create file");
            f.write_all(page.to_string().as_bytes()).expect("Failed to write message");                        
        }
    }
}
