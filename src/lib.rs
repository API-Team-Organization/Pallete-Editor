use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub struct Point {
    x: f64,
    y: f64,
}

#[wasm_bindgen]
impl Point {
    #[wasm_bindgen(constructor)]
    pub fn new (x: f64, y: f64) -> Point {
        Point { x, y }
    }

    #[wasm_bindgen(getter)]
    pub fn x(&self) -> f64 {
        self.x
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> f64 {
        self.y
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub struct Size {
    width: f64,
    height: f64,
}

#[wasm_bindgen]
impl Size {
    #[wasm_bindgen(constructor)]
    pub fn new(width: f64, height: f64) -> Size {
        Size { width, height}
    }

    #[wasm_bindgen(getter)]
    pub fn width(&self) -> f64 {
        self.width
    }

    #[wasm_bindgen(getter)]
    pub fn height(&self) -> f64 {
        self.height
    }
}

#[wasm_bindgen]
pub struct DraggableBlock {
    position: Point,
    size: Size,
    min_size: Size,
    is_dragging: bool,
    is_resizing: bool,
    drag_start: Point,
}

#[wasm_bindgen]
impl DraggableBlock {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> DraggableBlock {
        DraggableBlock {
            position: Point::new(x, y),
            size: Size::new(width, height),
            min_size: Size::new(100.0, 40.0),
            is_dragging: false,
            is_resizing: false,
            drag_start: Point::new(0.0, 0.0),
        }
    }

    pub fn start_drag(&mut self, mouse_x: f64, mouse_y: f64) {
        self.is_dragging = true;
        self.drag_start = Point::new(
            mouse_x - self.position.x,
            mouse_y - self.position.y,
        );
    }

    pub fn start_resize(&mut self, mouse_x: f64, mouse_y: f64) {
        self.is_resizing = true;
        self.drag_start = Point::new(mouse_x, mouse_y);
    }

    pub fn update(&mut self, mouse_x: f64, mouse_y: f64) {
        if self.is_dragging {
            self.position = Point::new(
                mouse_x - self.drag_start.x,
                mouse_y - self.drag_start.y,
            );
        } else if self.is_resizing {
            let dx: f64 = mouse_x - self.drag_start.x;
            let dy: f64 = mouse_y - self.drag_start.y;

            self.size = Size::new(
                (self.size.width + dx).max(self.min_size.width),
                (self.size.height + dy).max(self.min_size.height),
            );

            self.drag_start = Point::new(mouse_x, mouse_y);
        }
    }

    pub fn stop(&mut self) {
        self.is_dragging = false;
        self.is_resizing = false;
    }
}