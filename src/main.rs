use std::{
    fmt::Display,
    io::{stdout, Write},
    mem,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
    usize,
};

/*
* █
* ▓
* ▒
* ░
*
*/

enum DisplayCharacters {
    FULL,
    DARK,
    MEDIUM,
    LIGHT,
    EMPTY,
}

impl DisplayCharacters {
    fn value(&self) -> char {
        match self {
            DisplayCharacters::FULL => '█',
            DisplayCharacters::DARK => '▓',
            DisplayCharacters::MEDIUM => '▒',
            DisplayCharacters::LIGHT => '░',
            DisplayCharacters::EMPTY => ' ',
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Cell {
    a: f32,
    b: f32,
}

pub struct ReactionDiffusion {
    pub frame: Vec<Vec<char>>,
    pub grid: Vec<Vec<Cell>>,
    pub next: Vec<Vec<Cell>>,
    height: usize,
    width: usize,
    center_height: usize,
    center_width: usize,
    radius: usize,
}

impl ReactionDiffusion {
    pub const DIFFUSION_A: f32 = 1.0;
    pub const DIFFUSION_B: f32 = 0.5;
    pub const FEED: f32 = 0.055;
    pub const KILL: f32 = 0.062;
    pub const FILL_AMOUNT: f32 = 0.2;

    pub fn new(height: usize, width: usize) -> Self {
        const DEFAULT_CELL: Cell = Cell { a: 1.0, b: 0.0 };
        let empty_frame = vec![vec![DisplayCharacters::EMPTY.value(); width]; height];
        let empty_grid = vec![vec![DEFAULT_CELL.clone(); width]; height];

        let mut rd = ReactionDiffusion {
            frame: empty_frame.to_owned(),
            grid: empty_grid.clone(),
            next: empty_grid.clone(),
            height,
            width,
            center_height: height / 2,
            center_width: width / 2,
            radius: (((width / 2) as f32) * 0.2) as usize,
        };
        rd.init_start_grid();

        rd
    }

    // fill a circle in the center of the grid
    fn init_start_grid(&mut self) {
        for row in (self.center_width - self.radius)..(self.center_width + self.radius) {
            for item in (self.center_height - self.radius)..(self.center_height + self.radius) {
                self.grid[row][item].b = 1.0;
            }
        }
    }

    fn process_grid(&mut self) {
        /*
        for (let x = 1; x < width - 1; x++) {
            for (let y = 1; y < height - 1; y++) {
                let a = grid[x][y].a;
                let b = grid[x][y].b;
                next[x][y].a =
                a + DIFFUSION_A * laplaceA(x, y) - a * b * b + FEED * (1 - a);

                next[x][y].b =
                b + DIFFUSION_B * laplaceB(x, y) + a * b * b - (KILL + FEED) * b;

                next[x][y].a = constrain(next[x][y].a, 0, 1);
                next[x][y].b = constrain(next[x][y].b, 0, 1);
            }
        }
        */
        fn constrain(value: f32, min: f32, max: f32) -> f32 {
            if value < min {
                min
            } else if value > max {
                max
            } else {
                value
            }
        }

        for x in 1..self.height - 1 {
            for y in 1..self.width - 1 {
                let a = self.grid[x][y].a;
                let b = self.grid[x][y].b;
                let next_a: f32 = a + Self::DIFFUSION_A * self.laplace_a(x, y) - a * b * b
                    + Self::FEED * (1.0 - a);
                let next_b: f32 = b + Self::DIFFUSION_B * self.laplace_b(x, y) + a * b * b
                    - (Self::KILL + Self::FEED) * b;

                self.next[x][y].a = constrain(next_a, 0.0, 1.0);
                self.next[x][y].b = constrain(next_b, 0.0, 1.0);
            }
        }
    }

    fn swap(&mut self) {
        mem::swap(&mut self.grid, &mut self.next);
    }

    fn laplace_a(&self, x: usize, y: usize) -> f32 {
        let mut sum_a: f32 = 0.0;

        sum_a += self.grid[x][y].a * -1.0;
        sum_a += self.grid[x - 1][y].a * 0.2;
        sum_a += self.grid[x + 1][y].a * 0.2;
        sum_a += self.grid[x][y + 1].a * 0.2;
        sum_a += self.grid[x][y - 1].a * 0.2;
        sum_a += self.grid[x - 1][y - 1].a * 0.05;
        sum_a += self.grid[x + 1][y - 1].a * 0.05;
        sum_a += self.grid[x + 1][y + 1].a * 0.05;
        sum_a += self.grid[x - 1][y + 1].a * 0.05;

        sum_a
    }

    fn laplace_b(&self, x: usize, y: usize) -> f32 {
        let mut sum_b: f32 = 0.0;

        sum_b += self.grid[x][y].b * -1.0;
        sum_b += self.grid[x - 1][y].b * 0.2;
        sum_b += self.grid[x + 1][y].b * 0.2;
        sum_b += self.grid[x][y + 1].b * 0.2;
        sum_b += self.grid[x][y - 1].b * 0.2;
        sum_b += self.grid[x - 1][y - 1].b * 0.05;
        sum_b += self.grid[x + 1][y - 1].b * 0.05;
        sum_b += self.grid[x + 1][y + 1].b * 0.05;
        sum_b += self.grid[x - 1][y + 1].b * 0.05;

        sum_b
    }

    fn get_next_frame(&mut self) -> String {
        self.process_grid();

        for row in 0..self.height {
            for item in 0..self.width {
                let a = self.grid[row][item].a;
                let b = self.grid[row][item].b;
                let c = ((a - b) * 255.0).floor() as i32;
                let c = c.clamp(0, 255);

                self.frame[row][item] = match c {
                    0..=50 => DisplayCharacters::FULL.value(),
                    1..=101 => DisplayCharacters::DARK.value(),
                    102..=152 => DisplayCharacters::MEDIUM.value(),
                    153..=203 => DisplayCharacters::LIGHT.value(),
                    _ => DisplayCharacters::EMPTY.value(),
                };
            }
        }

        let mut frame_string = String::new();
        for row in 0..self.height {
            for item in 0..self.width {
                frame_string.push(self.frame[row][item]);
            }
            frame_string.push('\n');
        }

        self.swap();
        frame_string
    }
}

fn main() -> std::io::Result<()> {
    let mut rd = ReactionDiffusion::new(50 as usize, 50 as usize);
    let running = Arc::new(AtomicBool::new(false));

    let frame = rd.get_next_frame();

    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&running))?;

    let mut i: u32 = 1;
    let stdout = stdout();
    let mut handle = stdout.lock();

    while !running.load(Ordering::Relaxed) {
        // Clear terminal and set cursor to row 1 col 1
        handle.write_all(format!("{esc}[2J{esc}[1;1H", esc = 27 as char).as_bytes())?;
        handle.write_all(frame.as_bytes())?;
        handle.write_all(format!("{i}\n").as_bytes())?;
        handle.flush()?;
        i += 1;
        thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
