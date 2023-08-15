pub struct Tetromino {
    shape: Vec<i16>,
}

impl Tetromino {
    pub fn shape(&self) -> &Vec<i16> {
        &self.shape
    }
}

pub fn get_shapes() -> Vec<Tetromino> {
    vec![
        Tetromino {
            shape: vec![0, 10, 0, 0, 0, 10, 0, 0, 0, 10, 0, 0, 0, 10, 0, 0],
        }, // Straight line
        // ..X.
        // ..X.
        // ..X.
        // ..X.
        Tetromino {
            shape: vec![0, 0, 0, 0, 0, 10, 10, 0, 0, 10, 10, 0, 0, 0, 0, 0],
        }, // box
        // ....
        // .XX.
        // .XX.
        // ....
        Tetromino {
            shape: vec![0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 10, 0, 0, 10, 0, 0],
        }, // Tee
        // ....
        // ....
        // XXX.
        // .X..
        Tetromino {
            shape: vec![0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 10, 0, 0, 10, 10, 0],
        }, // Right Ell
        // ....
        // ..X.
        // ..X.
        // .XX.
        Tetromino {
            shape: vec![0, 0, 0, 0, 0, 10, 0, 0, 0, 10, 0, 0, 0, 10, 10, 0],
        }, // Left Ell
        // ....
        // .X..
        // .X..
        // .XX.
        Tetromino {
            shape: vec![0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 0, 0, 0, 10, 10, 0],
        }, // Ess
        // ....
        // ....
        // XX..
        // .XX.
        Tetromino {
            shape: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 10, 0, 10, 10, 0, 0],
        }, // Zee
           // ....
           // ....
           // .XX.
           // XX..
    ]
}
