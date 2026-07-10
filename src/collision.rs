use macroquad::prelude::*;
use crate::ball::Ball; 
use crate::block::Block; 
use std::cmp::Ordering; 

#[derive(Debug, PartialEq, Default)]
pub enum Dir {
    #[default] LR, 
    UD, 
}

#[derive(Debug, PartialEq, Default)]
pub struct CollisionState {
    pub hit_dir:Dir, 
    pub hit_time:Option<f32>, 
}

impl PartialOrd for CollisionState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.hit_time.unwrap_or(f32::INFINITY)
        .partial_cmp(&other.hit_time.unwrap_or(f32::INFINITY))
    }
}

impl CollisionState { 
    pub fn trash() -> CollisionState {
        CollisionState{hit_dir:Dir::LR, hit_time: None}
    }
}

pub fn get_block_collision_time(ball: &Ball, block: &Block, time_limit:f32) -> CollisionState{
    // ball.x -> ball.x + ball.vx * fps (get_frame_time()) 
    // 만날때까지 시간 게산하기 
    // [block.x-ball.r, block.x+block.w+ball.r] x [block.y-ball.r, block.y+block.h+ball.r] 에 도달? -> 아 time interval idea!!!! 
    let mut tx = (block.x - ball.r - ball.x) / ball.vx; 
    let mut txx = (block.x + block.w + ball.r - ball.x) / ball.vx; 
    let mut ty = (block.y - ball.r - ball.y) / ball.vy; 
    let mut tyy = (block.y + block.h + ball.r - ball.y) / ball.vy; 
    if tx>txx {
        let tmp = tx; 
        tx = txx; 
        txx = tmp; 
    } 

    if ty>tyy {
        let  tmp = ty; 
        ty = tyy; 
        tyy = tmp; 
    }
    // tx = tx.max(0.); txx = txx.max(0.); ty = ty.max(0.); tyy = tyy.max(0.); 
    if txx < 0.0 || tyy < 0.0 || tx > tyy || ty > txx{
        return CollisionState::trash(); 
    }
    let col_t = tx.max(ty); 
    if col_t > time_limit {
        return CollisionState::trash(); 
    }
    assert!(col_t>0.); 
    let dir_ = if tx > ty {
        Dir::LR
    } else {
        Dir::UD
    };
    CollisionState{hit_dir:dir_, hit_time:Some(col_t)}
}

pub fn get_wall_collision_time(ball: &Ball, time_limit:f32) -> CollisionState { 

    let tx = if ball.vx > 0. {
        (screen_width() - ball.x - ball.r) / ball.vx
    } else {
        (ball.x - ball.r) / ball.vx.abs()
    };

    let ty = if ball.vy > 0. {
        (screen_height() - ball.y - ball.r) / ball.vy
    } else {
        (ball.y - ball.r) / ball.vy.abs()
    };

    if tx.min(ty) > time_limit {
        return CollisionState::trash(); 
    } 

    let dir_ = if tx > ty {
        Dir::UD 
    } else {
        Dir::LR
    };

    CollisionState{hit_dir:dir_, hit_time:Some(tx.min(ty))}
}
