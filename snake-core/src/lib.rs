use wasm_bindgen::prelude::*;
use web_sys::{window, Document, HtmlCanvasElement, CanvasRenderingContext2d};
use js_sys::Math;

#[wasm_bindgen]
pub struct SnakeGame {
    ctx: CanvasRenderingContext2d,
    width: u32,
    height: u32,
    grid_size: u32,
    snake: Vec<(u32, u32)>,
    dir: (i32, i32),
    food: (u32, u32),
    game_over: bool,
    score: u32,
}

#[wasm_bindgen]
impl SnakeGame {
    /// Construct a new SnakeGame bound to <canvas id=canvas_id> with a grid size of `grid_size × grid_size`.
    /// Returns `Err(JsValue)` if we fail to find or cast the canvas/context.
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str, grid_size: u32) -> Result<SnakeGame, JsValue> {
        // 1) Get the Window and Document
        let win = window().ok_or_else(|| JsValue::from_str("Failed to get window"))?;
        let doc: Document = win
            .document()
            .ok_or_else(|| JsValue::from_str("Failed to get document"))?;

        // 2) Find <canvas id=canvas_id> and cast it
        let canvas = doc
            .get_element_by_id(canvas_id)
            .ok_or_else(|| JsValue::from_str("Canvas element not found"))?
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| JsValue::from_str("Element is not a HtmlCanvasElement"))?;

        // 3) Get the 2D rendering context
        let ctx = canvas
            .get_context("2d")
            .map_err(|_| JsValue::from_str("Failed to get context"))?
            .ok_or_else(|| JsValue::from_str("Context is None"))?
            .dyn_into::<CanvasRenderingContext2d>()
            .map_err(|_| JsValue::from_str("Cannot convert to CanvasRenderingContext2d"))?;

        // 4) Read the pixel dimensions of the canvas
        let width = canvas.width();
        let height = canvas.height();

        // 5) Initialize the snake at the center of the grid
        let start_x = grid_size / 2;
        let start_y = grid_size / 2;
        let mut snake = Vec::new();
        snake.push((start_x, start_y));

        // 6) Initial direction: moving right
        let dir = (1, 0);

        // 7) Place the initial food somewhere (placeholder)
        let food = (grid_size / 3, grid_size / 3);

        // Score
        let score = 0;

        Ok(SnakeGame {
            ctx,
            width,
            height,
            grid_size,
            snake,
            dir,
            food,
            game_over: false,
            score,
        })
    }

    /// Change the snake's direction. Prevent 180° reversal.
    pub fn change_direction(&mut self, dx: i32, dy: i32) {
        if (dx, dy) != (-self.dir.0, -self.dir.1) {
            self.dir = (dx, dy);
        }
    }

    /// Advance the game by one tick (move, check collisions, eat food).
    pub fn update(&mut self) {
        if self.game_over {
            return;
        }

        // 1) Compute new head in grid coords
        let (head_x, head_y) = self.snake[0];
        let new_x_i = head_x as i32 + self.dir.0;
        let new_y_i = head_y as i32 + self.dir.1;

        // 2) Wrap around edges
        let new_x = if new_x_i < 0 {
            self.grid_size - 1
        } else if new_x_i >= self.grid_size as i32 {
            0
        } else {
            new_x_i as u32
        };
        let new_y = if new_y_i < 0 {
            self.grid_size - 1
        } else if new_y_i >= self.grid_size as i32 {
            0
        } else {
            new_y_i as u32
        };

        // 3) Self-collision?
        if self.snake.contains(&(new_x, new_y)) {
            self.game_over = true;
            return;
        }

        // 4) Insert new head at front
        self.snake.insert(0, (new_x, new_y));

        // 5) If we ate food, grow and spawn new; else pop tail
        if (new_x, new_y) == self.food {
            self.score += 1;
            self.spawn_food();
        } else {
            self.snake.pop();
        }
    }

    /// Draw the current state: clear canvas, draw food + snake, draw "Game Over" if ended.
    pub fn render(&self) {
        // 1) Clear to black using the newer `set_fill_style_with_str`
        self.ctx.set_fill_style_str("black");
        self.ctx
            .fill_rect(0.0, 0.0, self.width as f64, self.height as f64);

        // 2) Compute cell size in pixels
        let cell_width = (self.width as f64) / (self.grid_size as f64);
        let cell_height = (self.height as f64) / (self.grid_size as f64);

        // 3) Draw food (red square) using `set_fill_style_with_str`
        self.ctx.set_fill_style_str("red");
        self.ctx.fill_rect(
            (self.food.0 as f64) * cell_width,
            (self.food.1 as f64) * cell_height,
            cell_width,
            cell_height,
        );

        // 4) Draw snake segments (green squares)
        self.ctx.set_fill_style_str("green");
        for &(x, y) in &self.snake {
            self.ctx.fill_rect(
                (x as f64) * cell_width,
                (y as f64) * cell_height,
                cell_width,
                cell_height,
            );
        }

        // Score
        self.ctx.set_fill_style_str("white");
        self.ctx.set_font("20px sans-serif");
        let score_text = format!("Score: {}", self.score);
        self.ctx.fill_text(&score_text, 10.0, 25.0).ok();

        // Game over
        if self.game_over {
            self.ctx.set_fill_style_str("white");
            self.ctx.set_font("40px sans-serif");
            let text = "Game Over";
            let text_x = (self.width as f64) / 2.0 - 100.0;
            let text_y = (self.height as f64) / 2.0;
            self.ctx.fill_text(text, text_x, text_y).ok();
        }
    }

    /// Getter so JS can see if the game has ended.
    #[wasm_bindgen(getter)]
    pub fn is_game_over(&self) -> bool {
        self.game_over
    }

    /// Pick a random empty cell for the next food.
    fn spawn_food(&mut self) {
        loop {
            // Math::random() ∈ [0,1), multiply by grid_size, floor to [0..grid_size-1]
            let fx = (Math::random() * (self.grid_size as f64)).floor() as u32;
            let fy = (Math::random() * (self.grid_size as f64)).floor() as u32;
            // If that spot is not on the snake, place food there
            if !self.snake.contains(&(fx, fy)) {
                self.food = (fx, fy);
                break;
            }
        }
    }
}
