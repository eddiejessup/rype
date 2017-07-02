use std::collections::HashMap;

use ::utils::{IntPoint, RealPoint};

pub struct Glyph {
    grid: Vec<Vec<bool>>,
}

impl Glyph {
    pub fn circle(radius: f64, page_nx: i32, page_ny: i32) -> Glyph {
        let h_box_nx = (2.0 * radius * (page_nx as f64)).ceil() as i32;
        let h_box_ny = (2.0 * radius * (page_ny as f64)).ceil() as i32;
        let mut grid = vec![vec![false; h_box_ny as usize]; h_box_nx as usize];

        for cell_i in 0..h_box_nx {
            for cell_j in 0..h_box_ny {
                let cell_x = (cell_i - h_box_nx / 2) as f64 / (page_nx as f64);
                let cell_y = (cell_j - h_box_ny / 2) as f64 / (page_ny as f64);
                let cell_r_sq = cell_x.powi(2) + cell_y.powi(2);
                grid[cell_i as usize][cell_j as usize] = cell_r_sq < radius.powi(2);
            }
        }
        Glyph {
            grid: grid,
        }
    }
}

pub struct CBox {
    glyph_name: String,
    origin: RealPoint,
}

pub struct HBox {
    pub c_boxes: Vec<CBox>,
    pub base_line: f64,
    pub base_line_k: f64,
    pub c_box_sep: f64,
    pub c_box_k: f64,
}

fn get_spring_force_mag(r: f64, k: f64) -> f64 {
    -k * r
}

impl HBox {
    pub fn iterate(&mut self, dt: f64) {
        for o1 in 0..self.c_boxes.len() {
            let mut f1 = RealPoint {x: 0.0, y: 0.0};
            for o2 in 0..self.c_boxes.len() {
                f1.add_inplace(&self.force_between(o1 as i32, o2 as i32));
            }
            f1.add_inplace(&self.force_line(o1));
            self.c_boxes[o1].origin.add_inplace(&f1.scale(dt));
        }
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();

        s += "\nnr_c_boxes:\n";
        s += &format!("{}\n\n", self.c_boxes.len());

        s += "glyph_name,x,y\n";
        for c_box in &self.c_boxes {
            s += &format!("{},{},{}\n", c_box.glyph_name, c_box.origin.x, c_box.origin.y);
        }
        s
    }

    pub fn add_c_box(&mut self, glyph_name: String, x: f64, y: f64) {
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
                x: get_spring_force_mag(self.c_boxes[o1 as usize].origin.x - (self.c_boxes[o2 as usize].origin.x + self.c_box_sep), self.c_box_k),
                y: 0.0,
            }
        } else if o2 == o1 - 1 {
            RealPoint {
                x: get_spring_force_mag(self.c_boxes[o1 as usize].origin.x - (self.c_boxes[o2 as usize].origin.x - self.c_box_sep), self.c_box_k),
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
            y: get_spring_force_mag(self.c_boxes[o1].origin.y - self.base_line, self.base_line_k),
        }
    }
}

pub struct Page {
    shape: IntPoint,
    pub h_boxes: Vec<HBox>,
    pub glyphs: HashMap<String,Glyph>,
}

impl Page {
    pub fn new(nx: i32, ny: i32) -> Page {
        Page {
            shape: IntPoint {i: nx, j: ny},
            h_boxes: vec![],
            glyphs: HashMap::new(),
        }
    }

    pub fn iterate(&mut self, dt: f64) {
        for h_box in &mut self.h_boxes {
            h_box.iterate(dt);
        }
    }

    pub fn to_string(&self) -> String {
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
