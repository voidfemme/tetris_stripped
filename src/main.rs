mod shapes;

use std::io as std_io;
use std::io::{Error, StdoutLock, Write};
use std::sync::mpsc;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{clear, cursor};

use shapes::{get_shapes, Tetromino};

const N_FIELD_WIDTH: u8 = 18;
const N_FIELD_HEIGHT: u8 = 18;
const LOOKUP: [char; 11] = [' ', 'A', 'B', 'C', 'D', 'F', 'G', '=', '#', '.', 'X'];

// A mock function to allow the code below to run
fn does_it_fit(
    n_tetromino: u8,
    n_rotation: u8,
    n_pos_y: u8,
    n_pos_x: u8,
    field: &Vec<Vec<u8>>,
) -> bool {
    true
}

fn rotate(px: u8, py: u8, r: u8) -> u8 {
    match r % 4 {
        0 => return py * 4 + px,
        1 => return 12 + py - (px * 4),
        2 => return 15 - (py * 4) - px,
        3 => return 3 - py + (px * 4),
        _ => return 0,
    }
}

fn display_message(
    handle: &mut RawTerminal<StdoutLock>,
    message: &str,
) -> Result<(), std::io::Error> {
    write!(
        handle,
        "{}{}",
        cursor::Goto(N_FIELD_WIDTH as u16 + 6, 2),
        message
    )?;
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    {
        let stdout = std_io::stdout();
        let mut handle = stdout.lock().into_raw_mode()?;
        write!(handle, "{}", cursor::Hide)?;
        handle.flush()?;

        // Create play field and play field buffer
        let mut field: Vec<Vec<u8>> =
            vec![vec![0; N_FIELD_WIDTH as usize]; N_FIELD_HEIGHT as usize];
        let mut field_buffer: Vec<Vec<u8>> =
            vec![vec![0; N_FIELD_WIDTH as usize]; N_FIELD_HEIGHT as usize];

        // Set up the game
        let tetrominos = get_shapes();
        let mut n_current_piece: u8 = 1;
        let mut n_current_rotation: u8 = 0;
        let mut n_current_x: u8 = N_FIELD_WIDTH / 2;
        let mut n_current_y: u8 = 0;
        let mut b_rotate_hold: bool = false;

        // Create a thread for handling input
        let (tx, rx) = mpsc::channel();
        let input_tx = tx.clone();
        let game_over = Arc::new(AtomicBool::new(false));
        let game_over_clone = Arc::clone(&game_over);

        // Spawn a thread to handle user input
        thread::spawn(move || {
            let result: Result<(), Box<dyn std::error::Error>> = (|| {
                let stdin = std_io::stdin();

                for key in stdin.keys() {
                    match key {
                        Ok(key) => {
                            input_tx.send(key)?;
                            if key == Key::Char('q') {
                                game_over_clone.store(true, Ordering::SeqCst);
                                break;
                            }
                        }
                        Err(err) => {
                            break;
                        }
                    }
                }
                Ok(())
            })();
            if let Err(e) = result {
                // Handle the error here
            }
        });

        // Clear the terminal before showing the play field.
        write!(handle, "{}", clear::All)?;
        // MAIN GAME LOOP
        loop {
            // TIMING ======================================
            sleep(Duration::from_millis(50));
            // INPUT =======================================
            match rx.try_recv() {
                Ok(key) => match key {
                    Key::Char('d') | Key::Right => {
                        if does_it_fit(
                            n_current_piece,
                            n_current_rotation,
                            n_current_x + 1,
                            n_current_y,
                            &field,
                        ) {
                            n_current_x += 1;
                            display_message(
                                &mut handle,
                                "'d' or 'Right'",
                            )?;
                        }
                    }
                    Key::Char('a') | Key::Left => {
                        if does_it_fit(
                            n_current_piece,
                            n_current_rotation,
                            n_current_x - 1,
                            n_current_y,
                            &field,
                        ) {
                            n_current_x -= 1;
                            display_message(
                                &mut handle,
                                "'a' or 'Left'",
                            )?;
                        }
                    }
                    Key::Char('s') | Key::Down => {
                        if does_it_fit(
                            n_current_piece,
                            n_current_rotation,
                            n_current_x,
                            n_current_y + 1,
                            &field,
                        ) {
                            n_current_y += 1;
                            display_message(
                                &mut handle,
                                "'s' or 'Down'",
                            )?;
                        }
                    }
                    Key::Char(' ') => {
                        if b_rotate_hold
                            && does_it_fit(
                                n_current_piece,
                                n_current_rotation + 1,
                                n_current_x,
                                n_current_y,
                                &field,
                            )
                        {
                            // Rotate, but latch to stop wild spinning
                            n_current_rotation += 1;
                            b_rotate_hold = false;
                        } else {
                            b_rotate_hold = true;
                            display_message(&mut handle, "rotate")?;
                        }
                    }
                    Key::Char('w') | Key::Up => {
                        if does_it_fit(
                            n_current_piece,
                            n_current_rotation,
                            n_current_x,
                            n_current_y + 1,
                            &field,
                        ) {
                            n_current_y -= 1;
                            display_message(
                                &mut handle,
                                "'w' or 'Up'",
                            )?;
                        }
                    }
                    _ => break,
                },
                Err(_e) => {
                    // No message this time, or an error occurred
                    // Just continue with the game loop
                }
            }

            // DISPLAY =====================================
        }
        Ok(())
    }
}
