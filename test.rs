use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

struct RealPoint {
    x: f64,
    y: f64,
}

impl RealPoint {
    fn mag_sq(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2)
    }

    fn mag(&self) -> f64 {
        self.mag_sq().sqrt()
    }

    fn sub(&self, other: &RealPoint) -> RealPoint {
        RealPoint {x: other.x - self.x, y: other.y - self.y}
    }

    fn add(&self, other: &RealPoint) -> RealPoint {
        RealPoint {x: other.x + self.x, y: other.y + self.y}
    }

    fn add_inplace(&mut self, other: &RealPoint) {
        self.x += other.x;
        self.y += other.y;
    }

    fn mul(&self, other: &RealPoint) -> RealPoint {
        RealPoint {x: other.x * self.x, y: other.y * self.y}
    }

    fn to_unit(&self) -> RealPoint {
        let mag = self.mag();
        RealPoint {x: self.x / mag, y: self.y / mag}
    }

    fn scale(&self, scale: f64) -> RealPoint {
        RealPoint {x: self.x * scale, y: self.y * scale}
    }

    fn round(&self) -> IntPoint {
        IntPoint {i: self.x.round() as i32, j: self.y.round() as i32}
    }
}


struct IntPoint {
    i: i32,
    j: i32,
}

impl IntPoint {
    fn mag_sq(&self) -> i32 {
        self.i.pow(2) + self.j.pow(2)
    }

    fn mag(&self) -> f64 {
        (self.mag_sq() as f64).sqrt()
    }

    fn sub(&self, other: &IntPoint) -> IntPoint {
        IntPoint {i: other.i - self.i, j: other.j - self.j}
    }

    fn add(&self, other: &IntPoint) -> IntPoint {
        IntPoint {i: other.i + self.i, j: other.j + self.j}
    }

    fn add_inplace(&mut self, other: &IntPoint) {
        self.i += other.i;
        self.j += other.j;
    }

    fn mul(&self, other: &IntPoint) -> IntPoint {
        IntPoint {i: other.i * self.i, j: other.j * self.j}
    }

    fn to_unit(&self) -> RealPoint {
        let mag = self.mag();
        RealPoint {x: self.i as f64 / mag, y: self.j as f64 / mag}
    }
}

struct Circle {
    // Between zero and one.
    radius: f64,
}

impl Circle {

    fn as_glyph(&self, page_nx: i32, page_ny: i32) -> Glyph {
        let h_box_nx = (2.0 * self.radius * (page_nx as f64)).ceil() as i32;
        let h_box_ny = (2.0 * self.radius * (page_ny as f64)).ceil() as i32;
        let mut grid = vec![vec![false; h_box_ny as usize]; h_box_nx as usize];

        for cell_i in 0..h_box_nx {
            for cell_j in 0..h_box_ny {
                let cell_x = (cell_i - h_box_nx / 2) as f64 / (page_nx as f64);
                let cell_y = (cell_j - h_box_ny / 2) as f64 / (page_ny as f64);
                let cell_r_sq = cell_x.powi(2) + cell_y.powi(2);
                grid[cell_i as usize][cell_j as usize] = cell_r_sq < self.radius.powi(2);
            }
        }
        Glyph {
            grid: grid,
        }
    }
}

struct Glyph {
    grid: Vec<Vec<bool>>,
}

struct CBox {
    glyph_name: String,
    origin: RealPoint,
}

struct HBox {
    c_boxes: Vec<CBox>,
    base_line: f64,
}

fn get_spring_force_mag(r: f64, k: f64) -> f64 {
    -k * r
}

const R_GLYPH: f64 = 0.1;
const K_GLYPH: f64 = 0.1;
const K_LINE: f64 = 0.05;
const R_LINE: f64 = 0.4;

impl HBox {
    fn add_c_box(&mut self, glyph_name: String, x: f64, y: f64) {
        let origin = RealPoint {
            x: x,
            y: y,
        };
        self.c_boxes.push(CBox {glyph_name: glyph_name, origin: origin});
    }

    fn force_between(&self, o1: i32, o2: i32) -> RealPoint {
        if o1 == o2 {
            RealPoint {
                x: 0.0,
                y: 0.0,
            }
        } else if o2 == o1 + 1 {
            RealPoint {
                x: get_spring_force_mag(self.c_boxes[o1 as usize].origin.x - (self.c_boxes[o2 as usize].origin.x + R_GLYPH), K_GLYPH),
                y: 0.0,
            }
        } else if o2 == o1 - 1 {
            RealPoint {
                x: get_spring_force_mag(self.c_boxes[o1 as usize].origin.x - (self.c_boxes[o2 as usize].origin.x - R_GLYPH), K_GLYPH),
                y: 0.0,
            }
        } else {
            RealPoint {
                x: 0.0,
                y: 0.0,
            }
        }
    }

    fn force_line(&self, o1: usize) -> RealPoint {
        RealPoint {
            x: 0.0,
            y: get_spring_force_mag(self.c_boxes[o1].origin.y - self.base_line, K_LINE),
        }
    }

    fn iterate(&mut self, dt: f64) {
        for o1 in 0..self.c_boxes.len() {
            let mut f1 = RealPoint {x: 0.0, y: 0.0};
            for o2 in 0..self.c_boxes.len() {
                f1.add_inplace(&self.force_between(o1 as i32, o2 as i32));
            }
            f1.add_inplace(&self.force_line(o1));
            self.c_boxes[o1].origin.add_inplace(&f1.scale(dt));
        }
    }

    fn to_string(&self) -> String {
        let mut s = String::new();

        s += "\nnr_c_boxes:\n";
        s += &format!("{}\n\n", self.c_boxes.len());

        s += "glyph_name,x,y\n";
        for c_box in &self.c_boxes {
            s += &format!("{},{},{}\n", c_box.glyph_name, c_box.origin.x, c_box.origin.y);
        }
        s
    }
}

struct Page {
    shape: IntPoint,
    h_boxes: Vec<HBox>,
    glyphs: HashMap<String,Glyph>,
}

impl Page {
    fn new(nx: i32, ny: i32) -> Page {
        Page {
            shape: IntPoint {i: nx, j: ny},
            h_boxes: vec![],
            glyphs: HashMap::new(),
        }
    }

    fn to_string(&self) -> String {
        let mut s = String::new();
        s += "grid_dimens:\n";
        s += &format!("nx,ny\n{},{}\n", self.shape.i, self.shape.j);

        s += "\nnr_glyphs:\n";
        s += &format!("{}\n\n", self.glyphs.len());

        for (glyph_name, glyph) in &self.glyphs {
            s += "glyph_name:\n";
            s += &format!("{}\n", glyph_name);

            s += "glyph_dimens:\n";
            s += &format!("gx,gy\n{},{}\n", glyph.grid.len(), glyph.grid[0].len());

            s += "glyph_shape:\n";
            for grid_row in &glyph.grid {
                let strs: Vec<_> = grid_row.iter().map(|&e| format!("{}", e as u8)).collect();
                s += &format!("{}\n", strs.join(","));
            }
        }

        s += &self.h_boxes[0].to_string();
        s
    }

}


fn main() {
    let args: Vec<String> = env::args().collect();
    let nx = 500;
    let ny = 500;

    let mut page = Page::new(nx, ny);

    let circ = Circle {
        radius: 0.02,
    };
    let circ_glyph = circ.as_glyph(nx, ny);

    let circ_glyph_name = String::from("circle");
    page.glyphs.insert(circ_glyph_name.clone(), circ_glyph);

    let mut h_box = HBox {
        c_boxes: vec![],
        base_line: R_LINE,
    };

    h_box.add_c_box(circ_glyph_name.clone(), 0.11, 0.23);
    h_box.add_c_box(circ_glyph_name.clone(), 0.67, 0.6);
    h_box.add_c_box(circ_glyph_name.clone(), 0.31, 0.4);
    h_box.add_c_box(circ_glyph_name.clone(), 0.77, 0.8);

    page.h_boxes.push(h_box);

    for t in 0..10000 {
        page.h_boxes[0].iterate(0.01);
        if t % 50 == 0 {
            // println!("hihi");
            let mut f = File::create(format!("dat/out_{0:010}.csv", t)).expect("Could not create file");
            f.write_all(page.to_string().as_bytes()).expect("Failed to write message");                        
        }
    }
}
