extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;


use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

use std::collections::LinkedList;
use std::iter::FromIterator;

struct Cell {
    gl: GlGraphics,
    alive: bool,
    cellShape: CellType,
    position: Point,
}
enum CellType {
    Triangle,
    Square,
    Hexagon
}

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // Change to OpenGL::V2_1 if fails.
    let opengl = OpenGL::V3_2;

    const COLS: u32 = 30;
    const ROWS: u32 = 20;
    const SQUARE_WIDTH: u32 = 20;

    let WIDTH = COLS * SQUARE_WIDTH;
    let HEIGHT = ROWS * SQUARE_WIDTH;

    let mut window: GlutinWindow = WindowSettings::new("Game of Life", [WIDTH, HEIGHT])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
}

