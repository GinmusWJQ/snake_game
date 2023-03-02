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
            direction: Direction::Up,
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

    /// `snake_head_idx`返回蛇头所处的index
    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }

    /** `update`根据蛇的的前进方向，找出蛇的下一个位置，并将下一个位置更新到自身`self.body[0].0`中
     *  #Example
     *  ```
     *  //直接调用即可
     *  self.update()
     *  ```
     * */
    pub fn update(&mut self) {
        let snake_idx = self.snake_head_idx();
        let (row, col) = self.index_to_cell(snake_idx);
        let (row, col) = match self.snake.direction {
            Direction::Right => (row, (snake_idx + 1) % self.width),
            Direction::Left => (row, (snake_idx - 1) % self.width),
            Direction::Up => ((row - 1) % self.width, col),
            Direction::Down => ((row + 1) % self.width, col),
            _ => (row, col),
        };
        let next_idx = self.cell_to_index(row, col);
        self.set_snake_head(next_idx);
    }

    /// `set_snake_head`接收index，并将该index更新到自身`self.body[0].0`中
    fn set_snake_head(&mut self, idx: usize) {
        self.snake.body[0].0 = idx;
    }

    /// 根据蛇在整个数组中所处的index,算出蛇所处的行和列,返回`(row,col)`
    fn index_to_cell(&self, index: usize) -> (usize, usize) {
        (index / self.width, index % self.width)
    }

    /// 根据蛇所处的行和列,算出蛇在整个数组中所处的index
    fn cell_to_index(&self, row: usize, col: usize) -> usize {
        (row * self.width) + col
    }
}
