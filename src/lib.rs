use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

// #[wasm_bindgen(module = "/www/utils/date.js")]
// extern {
//     fn now() -> usize;
// }
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

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum GameState {
    Won,
    Lost,
    Playing,
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
    reward_cell: Option<usize>,
    state: Option<GameState>,
    points: usize,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_idx: usize, snake_dir: Direction) -> World {
        let size = width * width;
        let snake = Snake::new(snake_idx, 3, snake_dir);
        World {
            width,
            size,
            reward_cell: World::generate_reward_cell(size, &snake.body),
            snake,
            next_cell: None,
            state: None,
            points: 0
        }
    }

    pub fn step(&mut self) {

        match self.state {
            Some(GameState::Playing) => {
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

                if self.snake.body[1..self.snake_length()].contains(&self.snake.body[0]) {
                    self.state = Some(GameState::Lost);
                } else {
                    if self.reward_cell == Some(self.snake_head_idx()) {
                        self.points += 1;
                        self.reward_cell =  World::generate_reward_cell(self.size, &self.snake.body);
                        if self.reward_cell.is_none() {
                            self.state = Some(GameState::Won);
                        } else {
                            self.snake.body.push(SnakeCell(self.snake.body[1].0));
                        }
                    }
    
                }        
            },
            _ => {}
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

    fn generate_reward_cell(max_val: usize, snake_body: &Vec<SnakeCell>) -> Option<usize> {
        if snake_body.len() < max_val {
            let mut reward_cell;
            loop {
                reward_cell = rnd(max_val);
                if !snake_body.contains(&SnakeCell(reward_cell)) { break; }
            }
            return Some(reward_cell);
        }
        None
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn points(&self) -> usize {
        self.points
    }

    pub fn get_reward_cell(&self) -> Option<usize> {
        self.reward_cell
    }

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn start_game(&mut self) {
        self.state = Some(GameState::Playing)
    }

    pub fn game_state(&self) -> Option<GameState> {
        self.state
    }

    pub fn game_state_text(&self) -> String {
        match self.state {
            Some(GameState::Won) => String::from("You have won!"),
            Some(GameState::Lost) => String::from("You have lost!"),
            Some(GameState::Playing) => String::from("You arer playing!"),
            None => String::from("Start playing!"),
        }
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