use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[wasm_bindgen]
pub struct SnakeCell(usize);
#[wasm_bindgen]
pub struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}
#[wasm_bindgen]
impl Snake {
    pub fn new(spawn_index: usize) -> Self {
        Snake {
            body: vec![SnakeCell(spawn_index)],
            direction: Direction::Right,
        }
    }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_idx: usize) -> Self {
        World {
            width,
            size: width * width,
            snake: Snake::new(snake_idx),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn update(&mut self) {
        let snake_idx = self.snake_head_idx();
        let row = snake_idx / self.width;
        match self.snake.direction {
            Direction::Right => {
                self.snake.body[0].0 = (snake_idx + 1) % self.size;
            }
            Direction::Left => {
                self.snake.body[0].0 = (snake_idx - 1) % self.size;
            }
            _ => {}
        }
    }
}
