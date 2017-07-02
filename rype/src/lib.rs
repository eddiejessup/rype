// use std::fs::File;
// use std::io::prelude::*;

// use doc::{Page, HBox};
// use utils::{Circle};

pub mod doc;
pub mod utils;

// const R_GLYPH: f64 = 0.1;
// const K_GLYPH: f64 = 0.1;
// const K_LINE: f64 = 0.05;
// const R_LINE: f64 = 0.4;

// fn main() {
//     let nx = 500;
//     let ny = 500;

//     let mut page = Page::new(nx, ny);

//     let circ = Circle {
//         radius: 0.02,
//     };
//     let circ_glyph = circ.as_glyph(nx, ny);

//     let circ_glyph_name = String::from("circle");
//     page.glyphs.insert(circ_glyph_name.clone(), circ_glyph);

//     let mut h_box = HBox {
//         c_boxes: vec![],
//         base_line: R_LINE,
//     };

//     h_box.add_c_box(circ_glyph_name.clone(), 0.11, 0.23);
//     h_box.add_c_box(circ_glyph_name.clone(), 0.67, 0.6);
//     h_box.add_c_box(circ_glyph_name.clone(), 0.31, 0.4);
//     h_box.add_c_box(circ_glyph_name.clone(), 0.77, 0.8);

//     page.h_boxes.push(h_box);

//     for t in 0..10000 {
//         page.h_boxes[0].iterate(0.01);
//         if t % 50 == 0 {
//             // println!("hihi");
//             let mut f = File::create(format!("dat/out_{0:010}.csv", t)).expect("Could not create file");
//             f.write_all(page.to_string().as_bytes()).expect("Failed to write message");                        
//         }
//     }
// }
