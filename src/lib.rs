use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(module = "/www/utils/date.js")]
extern {
    fn now() -> usize;
}
#[wasm_bindgen(module = "/www/utils/rnd.js")]
extern {
    fn rnd(max: usize) -> usize;
}

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}

#[derive(Clone, Copy, PartialEq)]
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
    snake: Snake,
    next_cell: Option<SnakeCell>,
    reward_cell: usize
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_idx: usize, snake_dir: Direction) -> World {
        let size = width * width;
        let snake = Snake::new(snake_idx, 3, snake_dir);
        let mut reward_cell;
        loop {
            reward_cell = rnd(size);
            if !snake.body.contains(&SnakeCell(reward_cell)) { break; }
        }
        World {
            width,
            size,
            snake,
            next_cell: None,
            reward_cell,
        }
    }

    pub fn step(&mut self) {
        let tmp = self.snake.body.clone();
        let len = tmp.len();
        self.snake.body[0] = match self.next_cell {
            Some(cell) => {
                self.next_cell = None;
                cell
            },
            None => self.gen_next_snake_cell()
        };

        for i in 1..len {
            self.snake.body[i] = SnakeCell(tmp[i-1].0)
        }
    }

    fn gen_next_snake_cell(&self) -> SnakeCell {
        self.get_next_snake_cell(&self.snake.direction)
    }

    fn get_next_snake_cell(&self, direction: &Direction) -> SnakeCell {
        let snake_idx = self.snake_head_idx();
        let row = snake_idx / self.width;
        // modula is computational expensive
        // let new_idx = match self.snake.direction {
        //     Direction::Right    => (row * self.width) + (snake_idx + 1) % self.width,
        //     Direction::Left     => (row * self.width) + (snake_idx - 1) % self.width,
        //     Direction::Up       => (snake_idx - self.width) % self.size,
        //     Direction::Down     => (snake_idx + self.width) % self.size,
        // };
        let new_idx = match direction {
            Direction::Right    => {
                let t = (row + 1) * self.width;
                if snake_idx + 1 == t {
                    t - self.width
                } else {
                    snake_idx + 1
                }
            },
            Direction::Left     => {
                let t = row * self.width;
                if snake_idx == t {
                    t + (self.width - 1)
                } else {
                    snake_idx - 1
                }
            },
            Direction::Up       => {
                let t = snake_idx - (row * self.width);
                if snake_idx == t {
                    (self.size - self.width) + t
                } else {
                    snake_idx - self.width
                }
            },
            Direction::Down     => {
                let t = snake_idx + ((self.width - row) * self.width);
                if snake_idx + self.width == t {
                    t - ((row + 1) * self.width)
                } else {
                    snake_idx + self.width
                }
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

    pub fn get_reward_cell(&self) -> usize {
        self.reward_cell
    }

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn change_snake_dir(&mut self, direction: Direction) {
        let next_cell = self.get_next_snake_cell(&direction);
        if self.snake.body[1].0 == next_cell.0 { return; }
        self.next_cell = Some(next_cell);
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