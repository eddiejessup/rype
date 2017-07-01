use std::env;
use std::f64::consts;

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
    fn area(&self) -> f64 {
        consts::PI * self.radius.powi(2)
    }

    fn perimeter(&self) -> f64 {
        2.0 * consts::PI * self.radius
    }

    fn as_glyph(&self, grid_nx: usize, grid_ny: usize) -> Glyph {
        let cbox_nx = (2.0 * self.radius * (grid_nx as f64)).ceil() as i32;
        let cbox_ny = (2.0 * self.radius * (grid_ny as f64)).ceil() as i32;
        let mut cbox_grid = vec![vec![false; cbox_ny as usize]; cbox_nx as usize];

        for cell_i in 0..cbox_nx {
            for cell_j in 0..cbox_ny {
                let cell_x = (cell_i - cbox_nx / 2) as f64 / (grid_nx as f64);
                let cell_y = (cell_j - cbox_ny / 2) as f64 / (grid_ny as f64);
                let cell_r_sq = cell_x.powi(2) + cell_y.powi(2);
                cbox_grid[cell_i as usize][cell_j as usize] = cell_r_sq < self.radius.powi(2);
            }
        }
        Glyph {
            grid: cbox_grid,
        }
    }
}


struct Glyph {
    grid: Vec<Vec<bool>>,
}

impl Glyph {
    fn to_c_box(self) -> CBox {
        CBox {
            origins: vec![],
            glyph: self,
        }
    }
}

struct CBox {
    origins: Vec<IntPoint>,
    glyph: Glyph,
}

impl CBox {
    fn add_origin(&mut self, x: f64, y: f64, nx: usize, ny: usize) {
        let origin = IntPoint {
            i: (x * nx as f64).round() as i32,
            j: (y * ny as f64).round() as i32,
        };
        self.origins.push(origin);
    }
}

fn get_force_mag(r_sq: i32) -> f64 {
    // TODO: Stub.
    1.0
}

fn force(du: IntPoint) -> RealPoint {
    let force_mag = get_force_mag(du.mag_sq());
    du.to_unit().scale(force_mag)
}

impl CBox {
    fn force_at(&self, o_from: usize, self_u: &IntPoint) -> RealPoint {
        let mut f = RealPoint {x: 0.0, y: 0.0};
        for i2 in 0..self.glyph.grid.len() {
            for j2 in 0..self.glyph.grid[0].len() {
                if self.glyph.grid[i2][j2] {
                    let other_u = self.pos(o_from, i2, j2);
                    let du = other_u.sub(self_u);
                    f.add_inplace(&force(du));
                }
            }
        }
        f
    }

    fn force_between(&self, o1: usize, o2: usize) -> RealPoint {
        let mut f = RealPoint {x: 0.0, y: 0.0};
        for i1 in 0..self.glyph.grid.len() {
            for j1 in 0..self.glyph.grid[0].len() {
                if self.glyph.grid[i1][j1] {
                    let self_u = self.pos(o1, i1, j1);
                    f.add_inplace(&self.force_at(o2, &self_u));
                }
            }
        }
        f
    }

    fn pos(&self, o: usize, i: usize, j: usize) -> IntPoint {
        IntPoint {i: self.origins[o].i + i as i32, j: self.origins[o].j + j as i32}
    }

    fn iterate(&self, dt: f64) {
        for o1 in 0..self.origins.len() {
            let mut f12 = RealPoint {x: 0.0, y: 0.0};
            for o2 in 0..self.origins.len() {
                if o1 == o2 {
                    continue;
                }
                f12.add_inplace(&self.force_between(o1, o2));
            }
            let dr = f12.scale(dt);
            let du = dr.round();
            self.origins[o1].add(&du);
        }
    }
}

fn add_cbox_to_grid(grid: &mut Vec<Vec<bool>>, cbox: &CBox) {
    for i in 0..cbox.glyph.grid.len() {
        for j in 0..cbox.glyph.grid[0].len() {
            if cbox.glyph.grid[i][j] {
                for origin_idx in 0..cbox.origins.len() {
                    let pos = cbox.pos(origin_idx, i, j);
                    grid[pos.i as usize][pos.j as usize] = true;
                }
            }
        }
    }
}

fn clear_grid(grid: &mut Vec<Vec<bool>>) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            grid[i][j] = false;
        }
    }
}

fn write_grid(grid: &Vec<Vec<bool>>) {
    for i in 0..grid.len() {
        let strs: Vec<_> = grid[i].iter().map(|&e| format!("{}", e as u8)).collect();
        print!("{}\n", strs.join(","));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let nx = 400;
    let ny = 400;

    let circ = Circle {
        radius: 0.05,
    };
    let circ_glyph = circ.as_glyph(nx, ny);
    let mut cbox = circ_glyph.to_c_box();
    cbox.add_origin(0.2, 0.2, nx, ny);
    cbox.add_origin(0.8, 0.8, nx, ny);

    // let mut grid = vec![vec![false; ny]; nx];
    for t in 0..20 {
        cbox.iterate(0.01);
        // clear_grid(&mut grid);
        // add_cbox_to_grid(&mut grid, &cbox);
        // write_grid(&grid);
    }
}
