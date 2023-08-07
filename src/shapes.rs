pub struct Tetromino {
    shape: Vec<u8>,
}

impl Tetromino {
    pub fn shape(&self) -> &Vec<u8> {
        &self.shape
    }
}

pub fn get_shapes() -> Vec<Tetromino> {
    vec![
        Tetromino {
            shape: vec![0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0],
        }, // Straight line
        // ..X.
        // ..X.
        // ..X.
        // ..X.
        Tetromino {
            shape: vec![0, 0, 0, 0, 0, 3, 3, 0, 0, 3, 3, 0, 0, 0, 0, 0],
        }, // box
        // ....
        // ....
        // XX..
        // XX..
        Tetromino {
            shape: vec![0, 0, 0, 0, 0, 0, 0, 0, 4, 4, 4, 0, 0, 4, 0, 0],
        }, // Tee
        // ....
        // ....
        // XXX.
        // .X..
        Tetromino {
            shape: vec![0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 4, 0, 0, 4, 4, 0],
        }, // Right Ell
        // ....
        // ..X.
        // ..X.
        // .XX.
        Tetromino {
            shape: vec![0, 0, 0, 0, 0, 5, 0, 0, 0, 5, 0, 0, 0, 5, 5, 0],
        }, // Left Ell
        // ....
        // .X..
        // .X..
        // .XX.
        Tetromino {
            shape: vec![0, 0, 0, 0, 0, 0, 0, 0, 6, 6, 0, 0, 0, 6, 6, 0],
        }, // Ess
        // ....
        // ....
        // XX..
        // .XX.
        Tetromino {
            shape: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 7, 0, 7, 7, 0, 0],
        }, // Zee
           // ....
           // ....
           // .XX.
           // XX..
    ]
}
