use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct SnakeCell(usize);
#[wasm_bindgen]
pub struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}
#[wasm_bindgen]
impl Snake {
    pub fn new(spawn_index: usize, direction: Direction, size: usize) -> Self {
        let mut body: Vec<_> = vec![];
        for i in 0..size {
            body.push(SnakeCell(spawn_index - i))
        }
        Snake { body, direction }
    }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
    reward_cell: usize,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_idx: usize, direction: Direction, snake_size: usize) -> Self {
        World {
            width,
            size: width * width,
            snake: Snake::new(snake_idx, direction, snake_size),
            reward_cell: 10,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn reward_cell(&self) -> usize {
        self.reward_cell
    }

    /// `snake_head_idx`返回蛇头所处的index
    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }

    /** `step`根据蛇的的前进方向，找出蛇的下一个位置，并将下一个位置更新到自身`self.snake.body`中
     *  #Example
     *  ```
     *  //直接调用即可
     *  self.step()
     *  ```
     * */
    pub fn step(&mut self) {
        let temp = self.snake.body.clone();
        let next_cell = self.gen_next_snake_cell(&self.snake.direction);
        self.snake.body[0] = next_cell;

        let len = self.snake.body.len();

        for i in 1..len {
            self.snake.body[i] = SnakeCell(temp[i - 1].0)
        }
    }

    ///生成下一个蛇单元
    fn gen_next_snake_cell(&self, direction: &Direction) -> SnakeCell {
        let snake_idx = self.snake_head_idx();
        let row = snake_idx / self.width;
        match direction {
            Direction::Right => SnakeCell((row * self.width) + (snake_idx + 1) % self.width),
            Direction::Left => SnakeCell((row * self.width) + (snake_idx - 1) % self.width),
            Direction::Up => SnakeCell((snake_idx - self.width) % self.size),
            Direction::Down => SnakeCell((snake_idx + self.width) % self.size),
        }
    }

    /// 获得蛇数组的首地址指针
    pub fn snake_cells(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }

    /// 获得蛇数组的长度
    pub fn snake_len(&self) -> usize {
        self.snake.body.len()
    }

    /// 改变蛇的方向
    pub fn change_snake_dir(&mut self, direction: Direction) {
        let next_cell = self.gen_next_snake_cell(&direction);
        if next_cell.0 != self.snake.body[1].0 {
            self.snake.direction = direction;
        }
    }
}
