use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

enum Direction {
    Up,
    Right,
    Down,
    Left
}

struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction
}

impl Snake {
    fn new(spawn_index: usize, dir: Direction) -> Snake {
        Snake {
            body: vec!(SnakeCell(spawn_index)),
            direction: dir

        }
    }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_idx: usize, snake_dir: u8) -> World {
        World {
            width,
            size: width * width,
            snake: Snake::new(snake_idx, match snake_dir {
                0 => Direction::Up,
                1 => Direction::Right,
                2 => Direction::Down,
                _ => Direction::Left
            })
        }
    }

    pub fn update(&mut self) {
        let snake_idx = self.snake_head_idx();
        let (row, col) = self.index_to_cell(snake_idx);
        let (row, col) = match self.snake.direction {
            Direction::Right    => (row, (col + 1) % self.width),
            Direction::Left     => (row, (col - 1) % self.width),
            Direction::Up       => ((row - 1) % self.width, col),
            Direction::Down     => ((row + 1) % self.width, col),
        };

        self.set_snake_head_idx(self.cell_to_index(row, col));
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }

    fn set_snake_head_idx(&mut self, idx: usize) {
        self.snake.body[0].0 = idx;
    }

    fn cell_to_index(&self, row: usize, col: usize) -> usize {
        (row * self.width) + col
    }

    fn index_to_cell(&self, idx: usize) -> (usize, usize) {
        (idx / self.width, idx % self.width)
    }
}