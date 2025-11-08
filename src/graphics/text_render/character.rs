use cgmath::{ Vector2, Point2 } ;

pub struct FontCharacter {
    pub position: Vector2<f32>,
    pub size: Vector2<f32>,
    pub bearing: Vector2<f32>,
    pub advance: f32
}