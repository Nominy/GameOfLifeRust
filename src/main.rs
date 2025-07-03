use raylib::prelude::*;
const WINDOW_WIDTH: i32 = 512;
const WINDOW_HEIGHT: i32 = 512;
const CELL_SIZE: i32 = 16;
const X: usize = (WINDOW_WIDTH/ CELL_SIZE) as usize;
const Y: usize = (WINDOW_HEIGHT/ CELL_SIZE) as usize;
const CELL_BG_COLOR_ALIVE: Color = Color::BLACK;
const CELL_BG_COLOR_DEAD: Color = Color::WHITE;
const CELL_FG_COLOR: Color = Color::new(127, 127, 127, 255);
const SPEED: f64 = 0.1;

fn count_neighbours(grid: [[bool; X]; Y], x: i32, y: i32) -> i32 {
    let neighbours = [
        (x -1, y -1), (x -1, y), (x -1, y +1),
        (x, y -1), (x, y +1),
        (x +1, y -1), (x +1, y), (x +1, y +1)
    ];
    let mut count = 0;
    for (x,y) in neighbours {
        let idx_x: usize = if x < 0 { X - 1 } else if x >= X as i32 { 0 } else { x as usize};
        let idx_y: usize = if y < 0 { Y - 1 } else if y >= Y as i32 { 0 } else { y as usize};
        if grid[idx_y][idx_x] {
            count += 1;
        }
    }
    count
}

fn check_neighbours(count: i32, state: bool) -> bool {
    if state && count <= 1 {
        false
    } else if state && count >= 4 {
        false
    } else if state && count >= 2 && count <= 3 {
        true
    } else if !state && count == 3 {
        true
    } else {
        state
    }
}

fn process_grid(grid: [[bool; X]; Y]) -> [[bool; X]; Y] {
    let mut new_grid = grid.clone();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let neighbour_count = count_neighbours(grid, j as i32, i as i32);
            let result = check_neighbours(neighbour_count, grid[i][j]);
            new_grid[i][j] = result;
        }
    }
    new_grid
}
fn draw_grid(grid: [[bool; X]; Y], mut d: RaylibDrawHandle) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let y = (i as i32)*CELL_SIZE;
            let x = (j as i32)*CELL_SIZE;

            let color = if grid[i][j] { CELL_BG_COLOR_ALIVE } else { CELL_BG_COLOR_DEAD };
            d.draw_rectangle(
                x,
                y,
                CELL_SIZE,
                CELL_SIZE,
                color
            );

            d.draw_rectangle_lines(
                x,
                y,
                CELL_SIZE,
                CELL_SIZE,
                CELL_FG_COLOR
            );
        }
    }
}

fn main() {

     let (mut rl, thread) = init()
         .size(WINDOW_WIDTH, WINDOW_HEIGHT)
         .title("Game of Life")
         .build();
     let mut grid = [[false;X];Y];
     grid[0][1] = true;
     grid[1][2] = true;
     grid[2][0] = true;
     grid[2][1] = true;
     grid[2][2] = true;
     let mut last_render_time = rl.get_time();
     let mut paused = false;


     while !rl.window_should_close() {
         let mut d = rl.begin_drawing(&thread);
         d.clear_background(Color::WHITE);
         if d.is_key_pressed(KeyboardKey::KEY_SPACE) {
             paused = !paused;
         }
         if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
             let mouse_x = d.get_mouse_x();
             let mouse_y = d.get_mouse_y();
             let x = mouse_x / CELL_SIZE;
             let y = mouse_y / CELL_SIZE;
             grid[y as usize][x as usize] = !grid[y as usize][x as usize];
         }
         if ((d.get_time() - last_render_time) > SPEED) && !paused {
             last_render_time = d.get_time();
             let new_grid = process_grid(grid);
             grid = new_grid;
         }
         draw_grid(grid, d);
     }
}