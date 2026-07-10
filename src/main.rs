use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "swiper".to_owned(),
        ..Default::default()
    }
}

struct Ball {
    x: f32, 
    y: f32, 
    r: f32,
    tex: Option<Texture2D>,
    vx: f32, 
    vy: f32, 
}


impl Ball {
    fn new(tex : Option<Texture2D>) -> Ball {
        Ball {
            x:60., 
            y:60.,
            r:40.,
            tex,
            vx:screen_width() * rand::gen_range(0.3,0.5), 
            vy:screen_height() * rand::gen_range(0.3,0.5), 
        }
    }

    fn draw(&self) {
        //println!("wtf?? {} {} {} {}",self.x, self.y, screen_height(), screen_width()); 
        let sides = 40; 
        let mut vertices = Vec::<Vertex>::with_capacity(sides as usize + 2);
        let mut indices = Vec::<u16>::with_capacity(sides as usize * 3);
        let rot = 0.0; 
        vertices.push(Vertex::new(self.x,self.y,0.,0.5,0.5,WHITE)); 
        for i in 0..=sides {
            let rx = (i as f32 / sides as f32 * std::f32::consts::PI * 2. + rot).cos();
            let ry = (i as f32 / sides as f32 * std::f32::consts::PI * 2. + rot).sin();

            let vertex = Vertex::new(self.x + self.r * rx, self.y + self.r * ry, 0., 0.5 + 0.5 * rx , 0.5 + 0.5 * ry , WHITE);

            vertices.push(vertex);

            if i != sides {
                indices.extend_from_slice(&[0, i as u16 + 1, i as u16 + 2]);
            }
        }
        let wtf : Mesh = Mesh { vertices: vertices, indices : indices, texture : Some(self.tex.as_ref().unwrap().clone())};
        draw_mesh(&wtf); 
    }

    fn check_hit_the_wall(&mut self) -> bool{
        let mut game_over:bool = false; 
        if self.x - self.r < 0. {
            self.vx = -self.vx;
        }

        if self.x + self.r > screen_width() {
            self.vx = -self.vx;
        }

        if self.y - self.r < 0. {
            self.vy = -self.vy;
        }

        if self.y + self.r > screen_height() {
            game_over = true; 
        }
        game_over 
    }

    fn move_ball(&mut self) {
        let fps : f32 = get_frame_time(); 
        self.x += self.vx * fps; 
        self.y += self.vy * fps; 
    }
}

struct Block {
    x: f32, 
    y: f32,
    w: f32, 
    h: f32, 
    tex: Option<Texture2D>, 
    hp: i32, 
    vx: f32,
}

impl Block {
    fn draw(&self) {
        let mut vertices = Vec::<Vertex>::new(); 
        let mut indices = Vec::<u16>::new(); 
        vertices.push(Vertex::new(self.x,self.y,0.,0.,0.,WHITE)); 
        vertices.push(Vertex::new(self.x, self.y + self.h,0., 0., 1.,WHITE)); 
        vertices.push(Vertex::new(self.x+self.w, self.y + self.h,0.,1.,1.,WHITE)); 
        vertices.push(Vertex::new(self.x+self.w,self.y,0.,1.,0.,WHITE));
        indices.extend_from_slice(&[0,1,2]); 
        indices.extend_from_slice(&[0,2,3]); 
        let wtf : Mesh = Mesh { vertices: vertices, indices : indices, texture : self.tex.clone()}; 
        draw_mesh(&wtf); 
    }
}

fn collision(ball:&mut Ball, block:&mut Block, score:&mut i32) {
    let cx = ball.x.clamp(block.x, block.x + block.w) - ball.x; 
    let cy = ball.y.clamp(block.y, block.y + block.h) - ball.y;
    let dis =  cx*cx + cy*cy - ball.r * ball.r; 
    println!("distance {} {:?} {:?}",dis,cx, cy); 
    if cx*cx + cy*cy - ball.r * ball.r < 0. {
        // 어떤 면에서 만나는지를 체크해야되는뎅. 
        // 위또는 아래 
        println!("kuromi hit!!!"); 
        *score += 1; 
        let up_hit:bool = ball.x >= block.x && ball.x <= block.x + block.w; 
        let lr_hit:bool = ball.y >= block.y && ball.y <= block.y + block.h; 
        if up_hit {
            ball.vy = -ball.vy * rand::gen_range(0.9, 1.1); 
        } 
        if lr_hit {
            ball.vx = -ball.vx * rand::gen_range(0.9, 1.1); 
        }
        block.hp -= 1 
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut dx : f32 = rand::gen_range(-6., 6.); 
    let mut dy : f32 = rand::gen_range(-6., 6.); 
    let rustacean_tex = load_texture("./kuromi.png").await.unwrap(); 
    let mut kuromi_ball : Ball = Ball::new(Some(rustacean_tex.clone())); 
    let mut stick : Block = Block{x:screen_width() / 2. - 30.0, y: screen_height() * 0.9, w:200.0, h:80.0, tex: Some(load_texture("./rustacean_happy.png").await.unwrap()), hp:10000, vx:100.}; 

    // draw texture 
    let mut game_over : bool = false; 
    let mut score = 0; 
    loop {
        if !game_over {
            clear_background(GRAY);
            let score_text = format!("score : {}",score); 
            let font_size = 20.; 
            let text_size = measure_text(&score_text, None, font_size as _, 1.0);

            draw_text(
                &score_text,
                10., 40., 
                font_size,
                DARKGRAY,
            );


            game_over = kuromi_ball.check_hit_the_wall(); 

            if is_key_down(KeyCode::Left) && stick.x > 0. {
                stick.x -= 4.0; 
            } 

            if is_key_down(KeyCode::Right) && stick.x + stick.w < screen_width() {
                stick.x += 4.0; 
            }   

            collision(&mut kuromi_ball,&mut stick,&mut score); 
            
            kuromi_ball.move_ball(); 
            kuromi_ball.draw(); 
            stick.draw(); 
        } 
        if game_over {
            clear_background(GRAY); 
            let text = format!("Game is over. Final score : {}. Press [Enter] to play again.",score); 
            let font_size = 30.; 
            let text_size = measure_text(&text, None, font_size as _, 1.0);

            draw_text(
                &text,
                screen_width() / 2. - text_size.width / 2.,
                screen_height() / 2. + text_size.height / 2.,
                font_size,
                DARKGRAY,
            );

            if is_key_down(KeyCode::Enter) {
                score = 0; 
                kuromi_ball = Ball::new(Some(rustacean_tex.clone())); 
                stick = Block{x:screen_width() / 2. - 30.0, y: screen_height() * 0.9, w:200.0, h:80.0, tex: Some(load_texture("./rustacean_happy.png").await.unwrap()), hp:10000, vx:100.}; 
                game_over = false; 
            }  
        }

        next_frame().await
    }
}