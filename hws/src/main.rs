extern crate router;
extern crate iron;
extern crate rype;

use std::sync::{Arc, RwLock};

use iron::prelude::*;
use iron::status;
use router::Router;

// use rype::utils::Point;
use rype::doc::{Page, HBox, Glyph};

fn state(_: &mut Request, page: &RwLock<Page>) -> IronResult<Response> {
    let page = page.read().unwrap();
    Ok(Response::with((status::Ok, format!("{}", page.to_string()))))
    // Ok(Response::with((status::Ok, format!("Number of H Boxes: {}", page.h_boxes.len()))))
}

fn iterate(_: &mut Request, page: &RwLock<Page>) -> IronResult<Response> {
    let mut page = page.write().unwrap();
    page.iterate(0.01);
    Ok(Response::with((status::Ok, "OK")))
}

const R_C_BOX: f64 = 0.1;
const K_C_BOX: f64 = 0.1;
const K_LINE: f64 = 0.05;
const R_LINE: f64 = 0.4;

fn initial_page() -> Page {
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
    page
}

fn main() {
    let page = Arc::new(RwLock::new(initial_page()));

    let mut router = Router::new();
    router.get("/state", { let page = page.clone(); move |req: &mut Request| state(req, &page) }, "state");
    router.post("/iterate", { let page = page.clone(); move |req: &mut Request| iterate(req, &page) }, "iterate");

    let _server = Iron::new(router).http("localhost:3000").unwrap();
    println!("On 3000");
}
