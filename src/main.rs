use std::{
    io::{stdout, Write},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

static DIFFUSION_A: f32 = 1.0;
static DIFFUSION_B: f32 = 0.5;
static FEED: f32 = 0.055;
static KILL: f32 = 0.062;
static GRID_LENGTH: usize = 50;
static GRID_HEIGHT: usize = 50;

// I want to use
fn init_grid(grid: &mut [[char; GRID_LENGTH]; GRID_HEIGHT]) {
    for row in 0..grid.len() {
        for item in 0..grid[row].len() {
            print!("{}", grid[row][item]);
        }
        print!("\n")
    }
}

fn main() -> std::io::Result<()> {
    let running = Arc::new(AtomicBool::new(false));

    let mut grid: [[char; GRID_LENGTH]; GRID_HEIGHT] = [['1'; GRID_LENGTH]; GRID_HEIGHT];
    init_grid(&mut grid);
    let mut next: [[char; GRID_LENGTH]; GRID_HEIGHT] = [['2'; GRID_LENGTH]; GRID_HEIGHT];
    init_grid(&mut next);

    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&running))?;

    let mut i: u32 = 1;
    let stdout = stdout();
    let mut handle = stdout.lock();

    while !running.load(Ordering::Relaxed) {
        let _ = handle.write_all(format!("{i}\n").as_bytes());
        handle.flush()?;
        i += 1;
        thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
