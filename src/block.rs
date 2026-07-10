use macroquad::prelude::*;
#[derive(Debug)]
pub struct Block {
    pub x: f32, 
    pub y: f32,
    pub w: f32, 
    pub h: f32, 
    pub tex: Option<Texture2D>, 
    pub hp: i32, 
    pub vx: f32,
}

impl Block {
    pub fn draw(&self) {
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

    pub fn move_block(&mut self) {
        if is_key_down(KeyCode::Left) {
            self.x -= self.vx * get_frame_time(); 
            if self.x < 0. {
                self.x = 0.; 
            }
        } 
        if is_key_down(KeyCode::Right) {
            self.x += self.vx * get_frame_time(); 
            if self.x + self.w > screen_width() {
                self.x = screen_width() - self.w; 
            }
        }   
    }
}