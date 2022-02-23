
use macroquad::prelude::*;
use macroquad::audio; 




use crate::player::*;  
use crate::enermy::*; 




struct Bullet {

}

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
    // bullet_vec: &mut Vec<Bullet>, 
    enermy_vec: &mut Vec<Enermy>,
    enermy_count: &mut i32, 
    score: &mut i32, 
    texture_bg: &Texture2D, 
    texture_player: Texture2D,
    texture_enermy: &Texture2D
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
        //spray bullet to his opponent
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


    for enermy in enermy_vec.iter_mut() {
        enermy.update(); 
        enermy.draw(*texture_enermy);  //referenced

    }


}