
use macroquad::audio;
use macroquad::audio::play_sound_once;
use macroquad::prelude::*;



use crate::player::*;  
use crate::enermy::*; 
use crate::bullet::*; 




pub fn bg_draw(texture: &Texture2D) {
    draw_texture(*texture, 0.0, 0.0, WHITE)
}


pub fn randomColor() -> Color {
    let color_vector: Vec<Color> = vec![PURPLE, ORANGE, PINK, LIME, BROWN, MAROON, GRAY, BLACK, WHITE]; 
    let color_index: usize = rand::gen_range(0, color_vector.len());
    color_vector[color_index]
  
}


pub fn render_score(score: &i32) {
    draw_text(
        &format!("Score: {}", score)[..],
        650.0,
        25.0,
        25.0,
        YELLOW
    );
}



pub fn logic(
    player: &mut Player, 
    bullet_vec: &mut Vec<Bullet>,
    enermy_vec: &mut Vec<Enermy>,
    enermy_count: &mut i32, 
    score: &mut i32, 
    texture_bg: &Texture2D, 
    texture_player: Texture2D,
    texture_enermy: &Texture2D, 
    weapon_audio: &audio::Sound, 
    enemydeath_audio: &audio::Sound, 
    gameover_audio: &audio::Sound
) {

    bg_draw(texture_bg);
    player.draw(texture_player);
    render_score(&score); 

    // for enerm in enermy_vec.iter() {
    //     // enerm.update()
    //     enerm.draw(*texture_enermy)
    // }

    if is_key_down(KeyCode::A) {
        player.left()
    }

    if(is_key_down(KeyCode::D)){
        player.right(); 
    }



    if(is_key_pressed(KeyCode::Space)) {
         bullet_vec.append( &mut vec![Bullet::new(player.x, player.y, 15.0, WHITE, true)]); //spray bullet to his opponent
         play_sound_once(*weapon_audio); 
    }

    if rand::rand() as i32 % 25 == 0 {
       if enermy_count >  &mut (enermy_vec.len() as i32) {
        enermy_vec.append(&mut vec![Enermy::new(rand::gen_range(0.0 + player.w, Conf::default().window_width as f32 - player.w), 20.0, rand::gen_range(1.0, 8.0), rand::gen_range(1.0, 8.0), randomColor(), false)]);
        *enermy_count += 1;
       }
    }

    if player.x > 800.0 - player.w {
        player.x = 800.0 - player.w; 
    }

    if player.x < 0.0 {
        player.x = 0.0; 
    }

    for bullet in bullet_vec.iter_mut() {
        if bullet.is_ready {
            bullet.fire();
        }

        bullet.update();
        bullet.draw(); 

        if bullet.y < 0.0 {
            bullet.ready()
        }
    }


    for enerm in enermy_vec.iter_mut() {
        enerm.update(); 
        enerm.draw(*texture_enermy);  //referenced

        if enerm.x > 800.0 - 15.0 {
            enerm.speed_x =- enerm.speed_x
        }

        if enerm.x < 0.0 {
            enerm.speed_x = -enerm.speed_x
        }
    }


    for bullet in bullet_vec.iter_mut() {
        for enerm in enermy_vec.iter_mut() {
            if enerm.x < bullet.x + 10.0 && enerm.x + 15.0 > bullet.x && enerm.y <bullet.y + 10.0 && enerm.y + 15.0 > bullet.y{
                play_sound_once(*enemydeath_audio);
                enerm.is_erased = true; 
                *score += 1;
            }

        }
 
    }


    for enerm in enermy_vec.iter_mut() {
        if player.x < enerm.x + 15.0 && player.x + 64.0 > enerm.x && player.y < enerm.y + 15.0 && player.y + 64.0 > enerm.y {
            player.gameover = true;
            play_sound_once(*gameover_audio); 
            break; 
        }
       
    }


    bullet_vec.retain(|x| x.y > 0.0); 
    enermy_vec.retain(|x| x.y < Conf::default().window_height as f32); 
    enermy_vec.retain(|x| !x.is_erased)
}