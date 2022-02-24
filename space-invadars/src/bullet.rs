use macroquad::prelude::*;


pub struct Bullet {
    pub x: f32, 
    pub y: f32, 
    pub speed: f32, 
    pub color: Color, 
    pub is_ready: bool
}


//x: f32, y: f32, speed_x: f32, speed_y: f32, color: Color, is_erased: bool
impl Bullet {

    pub fn new(x: f32, y: f32, speed: f32, color: Color, is_ready: bool) -> Self {
        Self {
            x, 
            y, 
            color, 
            speed,
            is_ready, 
           
        }
    }

    pub fn update(&mut self) {
        self.y -= self.speed; 
    }

    pub fn ready(&mut self){
        self.is_ready = true; 
    }


    pub fn draw(&self) {
        draw_rectangle(self.x as f32, self.y - 5.0 , 5.0, 15.0, self.color);
    }

    pub fn fire(&mut self) {
        self.is_ready = false; 
    }
}