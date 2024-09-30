#[derive(Copy, Clone, Default)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone, Default)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn sum(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

pub fn world_to_screen(matrix: &[f32; 16], pos: Vector3, screen: Vector2) -> Vector2 {
    let screen_w = (matrix[12] * pos.x) + (matrix[13] * pos.y) + (matrix[14] * pos.z) + matrix[15];

    if screen_w > 0.001 {
        let screen_x = (matrix[0] * pos.x) + (matrix[1] * pos.y) + (matrix[2] * pos.z) + matrix[3];
        let screen_y = (matrix[4] * pos.x) + (matrix[5] * pos.y) + (matrix[6] * pos.z) + matrix[7];

        let x = (screen.x / 2.0) + (screen.x / 2.0) * screen_x / screen_w;
        let y = (screen.y / 2.0) - (screen.y / 2.0) * screen_y / screen_w;

        Vector2 { x, y }
    } else {
        Vector2 { x: -99.0, y: -99.0 }
    }
}