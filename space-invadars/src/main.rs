

use macroquad::{prelude::*, audio::{load_sound_from_bytes}};


fn window_conf() -> Conf {
    Conf {
        window_resizable: false, 
        window_width: 800, 
        window_height: 600, 
        window_title: "SpaceInvadars".to_string(), ..Default::default()
    }
}



pub struct Player {
    pub x: f32, 
    pub y: f32, 
    pub w: f32, 
    pub speed: f32, 
    pub color: Color, 
    pub gameover: bool
}



impl Player {
    pub fn draw(&mut self, texture: Texture2D) {
        draw_texture(texture, self.x, self.y, self.color)
    }
}


#[macroquad::main(window_conf())]
async fn main() {
    // show_mouse(false); 
    let mut score:i32 = 0; 

    let mut player:Player = Player {
        x: Conf::default().window_width as f32 /2.0 - 6.0, 
        y: Conf::default().window_height as f32 / 2.0 -6.0, 
        speed: 5.0, 
        w: 60.0, 
        color: WHITE, 
        gameover: false
    };


    let texture_player = Texture2D::from_file_with_format(include_bytes!("../assets/spaceship.png"), None);
    let texture_bg = Texture2D::from_file_with_format(include_bytes!("../assets/bg.png"), None); 
    let texture_enermy = Texture2D::from_file_with_format(include_bytes!("../assets/enemy.png"), None);

    // let audio_explosion = load_sound_from_bytes(include_bytes!("../assets/explosion.wav")).await.unwrap(); 

    loop {
        if player.gameover {
            clear_background(BLACK); 
            draw_text("GAME OVER", Conf::default().window_width as f32 - 600.0, Conf::default().window_height as f32 / 2.0 + 25.0, 100.0, LIME);
            draw_text(&format!("Score: {}", score), Conf::default().window_width as f32 /2.0 - 45.0, Conf::default().window_height as f32/2.0 + 80.0, 25.0, YELLOW); 
            
        }else {
            //continue game here
            draw_text("GAME STARTED", Conf::default().window_width as f32 - 600.0, Conf::default().window_height as f32 / 2.0 + 25.0, 100.0, GREEN);
        }

        next_frame().await
    }
   
}
