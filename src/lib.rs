use wasm_bindgen::prelude::*;
use web_sys::{Element, MouseEvent};

#[wasm_bindgen]
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
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

    #[wasm_bindgen(getter)]
    pub fn position(&self) -> Point {
        self.position.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn size(&self) -> Size {
        self.size.clone()
    }
}