use macroquad::prelude::*; 
use crate::collision::{CollisionState,Dir}; 

#[derive(Debug)]
pub struct Ball {
    pub x: f32, 
    pub y: f32, 
    pub r: f32,
    pub tex: Option<Texture2D>,
    pub vx: f32, 
    pub vy: f32, 
}

impl Ball {
    pub fn new(tex : Option<Texture2D>) -> Ball {
        Ball {
            x:60., 
            y:60.,
            r:40.,
            tex,
            vx:screen_width() * rand::gen_range(0.3,0.5), 
            vy:screen_height() * rand::gen_range(0.3,0.5), 
        }
    }

    pub fn draw(&self) {
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

    pub fn move_ball(&mut self, fps : f32) {
        self.x += self.vx * fps; 
        self.y += self.vy * fps; 
    }

    pub fn process_collision_state(&mut self, collision_state : &CollisionState) { 
        self.move_ball(collision_state.hit_time.unwrap()); 
        match collision_state.hit_dir {
            Dir::LR => {
                self.vx = - self.vx 
            },  
            Dir::UD => {
                self.vy = - self.vy; 
            }, 
        }
        self.move_ball(0.0000001);  // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    }
}

