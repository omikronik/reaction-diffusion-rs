use std::{
    fmt::Display,
    io::{stdout, Write},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

/*
* █
* ▓
* ▒
* ░
*
*/

static DIFFUSION_A: f32 = 1.0;
static DIFFUSION_B: f32 = 0.5;
static FEED: f32 = 0.055;
static KILL: f32 = 0.062;
static GRID_LENGTH: usize = 50;
static GRID_HEIGHT: usize = 50;
static FILL_AMOUNT: f32 = 0.2;

static FULL: char = '█';
static DARK: char = '▓';
static MEDIUM: char = '▒';
static LIGHT: char = '░';
static EMPTY: char = ' ';

#[derive(Debug, Copy, Clone)]
struct Cell {
    a: f32,
    b: f32,
}

struct Grid {}

fn init_start_grid(grid: &mut [[Cell; GRID_LENGTH]; GRID_HEIGHT]) {
    // length = 50
    // find center, x/2 y/2 = 25,25
    // cover 30% means radius of length 15% of 25 =
    //let center_x = grid.len().div_floor(2);
    let center_x: usize = grid.len() / 2;
    let center_y: usize = grid[0].len() / 2;
    let radius = ((center_x as f32) * 0.2) as usize;
    for row in (center_x - radius)..(center_x + radius) {
        for item in (center_y - radius)..(center_y + radius) {
            grid[row][item].b = 1.0;
        }
    }
}

fn print_grid(grid: [[Cell; GRID_LENGTH]; GRID_HEIGHT]) {
    for row in 0..grid.len() {
        println!("\nROW: {row}");
        for item in 0..grid[row].len() {
            print!("{:?},{:?}|", grid[row][item].a, grid[row][item].b)
        }
    }
}

fn print_fancy_grid(grid: [[char; GRID_LENGTH]; GRID_HEIGHT]) {
    for row in 0..grid.len() {
        for item in 0..grid[row].len() {
            print!("{}", grid[row][item]);
        }
        println!();
    }
}

fn fancy_output(grid: [[Cell; GRID_LENGTH]; GRID_HEIGHT]) -> [[char; GRID_LENGTH]; GRID_HEIGHT] {
    let mut pixels = [[' '; GRID_LENGTH]; GRID_HEIGHT];

    for row in 0..grid.len() {
        for item in 0..grid[row].len() {
            let a = grid[row][item].a;
            let b = grid[row][item].b;
            let c = ((a - b) * 255.0).floor() as i32;
            let c = c.clamp(0, 255);

            pixels[row][item] = match c {
                0..=50 => FULL,
                51..=101 => DARK,
                102..=152 => MEDIUM,
                153..=203 => LIGHT,
                _ => EMPTY,
            };
        }
    }

    pixels
}

impl From<(f32, f32)> for Cell {
    fn from(t: (f32, f32)) -> Cell {
        Cell { a: t.0, b: t.1 }
    }
}

fn main() -> std::io::Result<()> {
    let running = Arc::new(AtomicBool::new(false));

    let mut grid: [[Cell; GRID_LENGTH]; GRID_HEIGHT] =
        [[(1.0, 0.0).into(); GRID_LENGTH]; GRID_HEIGHT];
    init_start_grid(&mut grid);
    let mut next: [[Cell; GRID_LENGTH]; GRID_HEIGHT] =
        [[(1.0, 0.0).into(); GRID_LENGTH]; GRID_HEIGHT];

    let fancy_grid = fancy_output(grid);
    print_fancy_grid(fancy_grid);

    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&running))?;

    let mut i: u32 = 1;
    let stdout = stdout();
    let mut handle = stdout.lock();

    while !running.load(Ordering::Relaxed) {
        let _ = handle.write_all(format!("{i}\n").as_bytes());
        handle.flush()?;
        i += 1;
        thread::sleep(Duration::from_millis(1000));
    }

    Ok(())
}
