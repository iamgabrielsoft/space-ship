

use macroquad::{prelude::*, audio::{load_sound_from_bytes}};


mod game_logic; 
mod player; 
mod enermy;
mod bullet; 


use player::*; 
use enermy::*;
use bullet::*;

fn window_conf() -> Conf {
    Conf {
        window_resizable: false, 
        window_width: 800, 
        window_height: 600, 
        window_title: "SpaceInvadars".to_string(), ..Default::default()
    }
}






#[macroquad::main(window_conf())]
async fn main() {
    // show_mouse(false); 
    let mut score: i32 = 0;
    let mut enermy_vec: Vec<Enermy> = vec![];
    let mut bullet_vec: Vec<Bullet> = vec![]; 
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
    let mut enermy_count: i32 = 1;  
    loop {
        if player.gameover {
            clear_background(BLACK); 
            draw_text("GAME OVER", Conf::default().window_width as f32 - 600.0, Conf::default().window_height as f32 / 2.0 + 25.0, 100.0, LIME);
            draw_text(&format!("Score: {}", score), Conf::default().window_width as f32 /2.0 - 45.0, Conf::default().window_height as f32/2.0 + 80.0, 25.0, YELLOW); 
            
        }else {
            //continue game here
            game_logic::logic(&mut player, &mut enermy_vec, &mut enermy_count, &mut score,  &texture_bg, texture_player, &texture_enermy); 
        }

        next_frame().await
    }
   
}
