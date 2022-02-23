use macroquad::prelude::*;


pub struct Bullet {
    pub x: i32, 
    pub y: f32, 
    pub speed: i32, 
    pub color: Color, 
    pub is_ready: bool
}



impl Bullet {
    pub fn draw(&self) {
        draw_rectangle(self.x as f32, self.y - 5.0 , 5.0, 15.0, self.color);
    }

    pub fn fire(&mut self) {
        self.is_ready = false; 
    }
}