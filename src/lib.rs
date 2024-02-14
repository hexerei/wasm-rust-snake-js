use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}

#[derive(Clone)]
pub struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction
}

impl Snake {
    fn new(spawn_index: usize, size: usize, dir: Direction) -> Snake {
        let mut body = Vec::new();
        for i in 0..size {
            body.push(SnakeCell(spawn_index - i));
        }
        Snake {
            body,
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
    pub fn new(width: usize, snake_idx: usize, snake_dir: Direction) -> World {
        World {
            width,
            size: width * width,
            snake: Snake::new(snake_idx, 3, snake_dir)
        }
    }

    pub fn step(&mut self) {
        let tmp = self.snake.body.clone();
        let len = tmp.len();
        let next_cell = self.gen_next_snake_cell();
        self.snake.body[0] = next_cell;

        for i in 1..len {
            self.snake.body[i] = SnakeCell(tmp[i-1].0)
        }
    }

    fn gen_next_snake_cell(&self) -> SnakeCell {
        let snake_idx = self.snake_head_idx();
        let row = snake_idx / self.width;
        // modula is computational expensive
        // let new_idx = match self.snake.direction {
        //     Direction::Right    => (row * self.width) + (snake_idx + 1) % self.width,
        //     Direction::Left     => (row * self.width) + (snake_idx - 1) % self.width,
        //     Direction::Up       => (snake_idx - self.width) % self.size,
        //     Direction::Down     => (snake_idx + self.width) % self.size,
        // };
        let new_idx = match self.snake.direction {
            Direction::Right    => {
                let t = (row + 1) * self.width;
                if snake_idx + 1 == t { t - self.width }
                                 else { snake_idx + 1 }
            },
            Direction::Left     => {
                let t = row * self.width;
                if snake_idx == t { t + (self.width - 1) }
                             else { snake_idx - 1 }
            },
            Direction::Up       => {
                let t = snake_idx - (row * self.width);
                if snake_idx == t { (self.size - self.width) + t }
                             else { snake_idx - self.width }
            },
            Direction::Down     => {
                let t = snake_idx - ((self.width - row) * self.width);
                if snake_idx + self.width == t { t - ((row + 1) * self.width) }
                             else { snake_idx + self.width }
            },
        };
        SnakeCell(new_idx)
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

    pub fn change_snake_dir(&mut self, direction: Direction) {
        self.snake.direction = direction;
    }

    pub fn snake_length(&self) -> usize {
        self.snake.body.len()
    }

    // *const is raw pointer not applying borrowing rules
    pub fn snake_cells(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()

    }

}