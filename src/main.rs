mod shapes;

use fern::Dispatch;
use log::info;

use std::io as std_io;
use std::io::Write;
use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, cursor};

use shapes::get_shapes;

const N_FIELD_WIDTH: i16 = 18;
const N_FIELD_HEIGHT: i16 = 18;
const LOOKUP: [char; 11] = [' ', 'A', 'B', 'C', 'D', 'F', 'G', '=', '#', '.', 'X'];

fn setup_logger(log_file: &str) -> Result<(), fern::InitError> {
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] {}:{} {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.file().unwrap_or("<unknown>"),
                record.line().unwrap_or(0),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(fern::log_file(log_file)?)
        .apply()?;
    Ok(())
}

fn does_it_fit(
    n_tetromino: i16,
    n_rotation: i16,
    n_pos_y: i16,
    n_pos_x: i16,
    field: &Vec<Vec<i16>>,
) -> bool {
    let tetrominos = get_shapes();
    for px in 0..4 {
        for py in 0..4 {
            // Get index into piece
            let pi = rotate(px, py, n_rotation);

            // Check that test is in bounds. Note out of bounds does not necessarily mean a fail,
            // as the long vertical piece can have cells that lie outside the boundary, so we'll
            // just ignore them.
            if n_pos_x + px >= 0 && n_pos_x + px < N_FIELD_WIDTH {
                if n_pos_y + py >= 0 && n_pos_y + py < N_FIELD_HEIGHT {
                    // In Bounds so do collision Check
                    if tetrominos[n_tetromino as usize].shape()[pi as usize] != 0
                        && field[(n_pos_y + py) as usize][(n_pos_x + px) as usize] != 0
                    {
                        info!(
                            "Collision check failure at ({}, {}): Found character '{}'",
                            px,
                            py,
                            LOOKUP
                                [field[(n_pos_y + py) as usize][(n_pos_x + px) as usize] as usize],
                        );
                        return false; // Fail on first hit
                    }
                }
            } else {
                info!(
                    "Skipping ({}, {}): Not in bounds",
                    n_pos_x + px,
                    n_pos_y + py
                );
            }
        }
    }
    true
}

fn rotate(px: i16, py: i16, r: i16) -> i16 {
    match r % 4 {
        0 => return py * 4 + px,
        1 => return 12 + py - (px * 4),
        2 => return 15 - (py * 4) - px,
        3 => return 3 - py + (px * 4),
        _ => return 0,
    }
}

fn main() -> Result<(), std::io::Error> {
    {
        setup_logger("output.log").expect("Failed to initialize logger");
        let stdout = std_io::stdout();
        let mut handle = stdout.lock().into_raw_mode()?;
        write!(handle, "{}", cursor::Hide)?;
        handle.flush()?;

        // Create play field and play field buffer
        let mut field: Vec<Vec<i16>> =
            vec![vec![0; N_FIELD_WIDTH as usize]; N_FIELD_HEIGHT as usize];
        let mut _field_buffer: Vec<Vec<i16>> =
            vec![vec![0; N_FIELD_WIDTH as usize]; N_FIELD_HEIGHT as usize];

        // Set up the game
        let tetrominos = get_shapes();
        let n_current_piece: i16 = 2;
        let mut n_current_rotation: i16 = 0;
        let mut n_current_x: i16 = N_FIELD_WIDTH / 2;
        let mut n_current_y: i16 = 0;
        let mut b_rotate_hold: bool = true;
        let mut b_game_over: bool = false;

        // Create a thread for handling input
        let (tx, rx) = mpsc::sync_channel(5);
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
                            info!("key {:#?} detected in input capture thread.", key);
                            if key == Key::Char('q') {
                                info!("'q' key recieved; quitting...");
                                game_over_clone.store(true, Ordering::SeqCst);
                                break;
                            }
                        }
                        Err(_err) => {
                            break;
                        }
                    }
                }
                Ok(())
            })();
            if let Err(_err) = result {
                // Handle the error here
            }
        });

        // Clear the terminal before showing the play field.
        write!(handle, "{}", clear::All)?;
        // MAIN GAME LOOP
        loop {
            // TIMING ======================================
            sleep(Duration::from_millis(25));
            // INPUT =======================================
            match rx.try_recv() {
                Ok(key) => {
                    match key {
                        Key::Char('d') | Key::Right => {
                            info!("Main thread recieved <right>/'d' key");
                            if does_it_fit(
                                n_current_piece,
                                n_current_rotation,
                                n_current_x + 1,
                                n_current_y,
                                &field,
                            ) {
                                n_current_x += 1;
                                info!("x = {n_current_x}, y = {n_current_y}");
                            } else {
                                info!("piece does not fit");
                            }
                        }
                        Key::Char('a') | Key::Left => {
                            info!("Main thread recieved <left>/'a' key");
                            if does_it_fit(
                                n_current_piece,
                                n_current_rotation,
                                n_current_x - 1,
                                n_current_y,
                                &field,
                            ) {
                                n_current_x -= 1;
                                info!("x = {n_current_x}, y = {n_current_y}");
                            } else {
                                info!("piece does not fit");
                            }
                        }
                        Key::Char('s') | Key::Down => {
                            info!("Main thread recieved <down>/'s' key");
                            if does_it_fit(
                                n_current_piece,
                                n_current_rotation,
                                n_current_x,
                                n_current_y + 1,
                                &field,
                            ) {
                                n_current_y += 1;
                                info!("x = {n_current_x}, y = {n_current_y}");
                            } else {
                                info!("piece does not fit");
                            }
                        }
                        Key::Char(' ') => {
                            info!("Main thread recieved <space> key");
                            if b_rotate_hold
                                && does_it_fit(
                                    n_current_piece,
                                    n_current_rotation + 1,
                                    n_current_x,
                                    n_current_y,
                                    &field,
                                )
                            {
                                info!("rotating piece: b_rotate_hold = {b_rotate_hold}");
                                // Rotate, but latch to stop wild spinning
                                n_current_rotation += 1;
                                b_rotate_hold = false;
                            } else {
                                info!("rotating piece: b_rotate_hold = {b_rotate_hold}");
                                b_rotate_hold = true;
                                info!("piece cannot rotate");
                            }
                        }
                        Key::Char('w') | Key::Up => {
                            info!("Main thread recieved <up>/'w' key");
                            if does_it_fit(
                                n_current_piece,
                                n_current_rotation,
                                n_current_x,
                                n_current_y + 1,
                                &field,
                            ) {
                                n_current_y -= 1;
                                info!("n_current_y = {n_current_y}");
                            } else {
                                info!("piece does not fit");
                            }
                        }
                        _ => b_game_over = true,
                    }
                }
                Err(TryRecvError::Empty) => {
                    // No key pressed
                }
                Err(TryRecvError::Disconnected) => {
                    info!("Input thread disconnected, exiting.");
                    break;
                }
            }

            if b_game_over {
                break;
            }

            // DISPLAY =====================================

            // Draw Border
            for px in 0..N_FIELD_WIDTH {
                for py in 0..N_FIELD_HEIGHT {
                    if px == 0 || px == N_FIELD_WIDTH - 1 || py == 0 || py == N_FIELD_HEIGHT - 1 {
                        field[px as usize][py as usize] = 8;
                    }
                }
            }
            // Draw tetromino
            // Iterate over the tetromino piece vector and if the cell is not '0' write the LOOKUP value to
            // the field. This has the effect of setting the values of only the cells that
            // represent the piece.
            for px in 0..4 {
                for py in 0..4 {
                    if (tetrominos[n_current_piece as usize].shape()
                        [rotate(px, py, n_current_rotation) as usize])
                        != 10
                    {
                        field[(n_current_y + py) as usize][(n_current_x + px) as usize] =
                            n_current_piece;
                    }
                }
            }

            // Draw field
            write!(handle, "{}", clear::All)?;
            for (y, row) in field.iter().enumerate() {
                for (x, &ch) in row.iter().enumerate() {
                    write!(
                        handle,
                        "{}{}",
                        cursor::Goto(x as u16 + 2, y as u16 + 2),
                        LOOKUP[ch as usize]
                    )?;
                    handle.flush()?;
                }
            }
        } // END MAIN LOOP
        write!(handle, "{}", cursor::Show)?;
        handle.flush()?;
    }
    Ok(())
}
