#[repr(u32)]
enum MazeCell {
    Ground = 0,
    Wall = 1,
}

pub struct Maze {
    data: [[f32; 200]; 200],
}

impl Maze {
    pub fn new() -> Self {
        let mut data = [[-0.01; 200]; 200];

        data[102][103] = 4.0;
        data[102][103] = 4.0;
        data[102][102] = 4.0;
        data[101][101] = 4.0;

        Self { data }
    }

    pub fn get_raw(&self) -> &[[f32; 200]; 200] {
        &self.data
    }
}
