mod shapes;

use fern::Dispatch;
use log::{info, warn};

use std::io as std_io;
use std::io::Write;
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
use termion::raw::IntoRawMode;
use termion::{clear, cursor};

use shapes::get_shapes;

const N_FIELD_WIDTH: u8 = 18;
const N_FIELD_HEIGHT: u8 = 18;
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
    n_tetromino: u8,
    n_rotation: u8,
    n_pos_y: i16,
    n_pos_x: i16,
    field: &Vec<Vec<u8>>,
) -> bool {
    let tetrominos = get_shapes();
    for px in 0..4 {
        for py in 0..4 {
            // Get index into piece
            let pi = rotate(px, py, n_rotation);

            // Check that test is in bounds. Note out of bounds does not necessarily mean a fail,
            // as the long vertical piece can have cells that lie outside the boundary, so we'll
            // just ignore them.
            if (n_pos_x + px as i16) < N_FIELD_WIDTH.into() {
                if (n_pos_y + py as i16) < N_FIELD_HEIGHT.into() {
                    // In Bounds so do collision Check
                    if tetrominos[n_tetromino as usize].shape()[pi as usize] != 0
                    // 10 is the index of the LOOKUP const
                        && field[(n_pos_y + py as i16) as usize][(n_pos_x + px as i16) as usize] != 0
                    {
                        return false; // Fail on first hit
                    }
                }
            }
        }
    }
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

fn main() -> Result<(), std::io::Error> {
    let _n_score: i32 = 0;
    {
        setup_logger("output.log").expect("Failed to initialize logger");
        let stdout = std_io::stdout();
        let mut handle = stdout.lock().into_raw_mode()?;
        write!(handle, "{}", cursor::Hide)?;
        handle.flush()?;

        // Create play field and play field buffer
        let mut field: Vec<Vec<u8>> =
            vec![vec![0; N_FIELD_WIDTH as usize]; N_FIELD_HEIGHT as usize];

        // Set up the game
        let tetrominos = get_shapes();
        let n_current_piece: u8 = 4;
        let mut n_current_rotation: u8 = 0;
        let mut n_current_x: i16 = 0;
        let mut n_current_y: i16 = 0;
        let mut b_rotate_hold: bool = false;

        // Create a thread for handling input
        let (tx, rx) = mpsc::channel();
        let input_tx = tx.clone();
        let game_over = Arc::new(AtomicBool::new(false));
        let game_over_clone = Arc::clone(&game_over);

        // Spawn a thread to handle user input
        thread::spawn(move || {
            let result: Result<(), Box<dyn std::error::Error>> = (|| {
                info!("Spawned new thread!");
                let stdin = std_io::stdin();

                for key in stdin.keys() {
                    match key {
                        Ok(key) => {
                            info!("Input handling thread detected {:#?} input", key);
                            input_tx.send(key)?;
                            if key == Key::Char('q') {
                                game_over_clone.store(true, Ordering::SeqCst);
                                break;
                            }
                        }
                        Err(err) => {
                            info!("Input error: {}", err);
                            break;
                        }
                    }
                }
                Ok(())
            })();
            if let Err(err) = result {
                // Handle the error here
                warn!("An error occurred: {}", err);
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
                    Key::Right => {
                        if does_it_fit(
                            n_current_piece,
                            n_current_rotation,
                            n_current_x + 1,
                            n_current_y,
                            &field,
                        ) {
                            info!("'d' pressed; n_current_x = {n_current_x}");
                            n_current_x += 1;
                        }
                    }
                    Key::Left => {
                        if does_it_fit(
                            n_current_piece,
                            n_current_rotation,
                            n_current_x - 1,
                            n_current_y,
                            &field,
                        ) {
                            info!("'a' pressed; n_current_x = {n_current_x}");
                            n_current_x -= 1;
                        }
                    }
                    Key::Down => {
                        if does_it_fit(
                            n_current_piece,
                            n_current_rotation,
                            n_current_x,
                            n_current_y + 1,
                            &field,
                        ) {
                            info!("'s' pressed; n_current_y = {n_current_y}");
                            n_current_y += 1;
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
                            info!("'<space>' pressed; n_current_rotation = {n_current_rotation}");
                            // Rotate, but latch to stop wild spinning
                            n_current_rotation += 1;
                            b_rotate_hold = false;
                        } else {
                            b_rotate_hold = true;
                        }
                    }
                    Key::Up => {
                        if does_it_fit(
                            n_current_piece,
                            n_current_rotation,
                            n_current_x,
                            n_current_y + 1,
                            &field,
                        ) {
                            info!("'w' pressed; n_current_y = {n_current_y}");
                            n_current_y -= 1;
                        }
                    }
                    _ => break,
                },
                Err(e) => {
                    // No message this time, or an error occurred
                    // Just continue with the game loop
                    warn!("Error: No message from rx this time: {}", e);
                }
            }

            // DISPLAY =====================================

            // Draw tetromino
            // Iterate over the tetromino piece vector and if the cell is not '0' write the LOOKUP value to
            // the field. This has the effect of setting the values of only the cells that
            // represent the piece.
            for px in 0..4 {
                for py in 0..4 {
                    if (tetrominos[n_current_piece as usize].shape()
                        [rotate(px, py, n_current_rotation) as usize])
                        != 0
                    {
                        field[(n_current_y + py as i16) as usize]
                            [(n_current_x + px as i16) as usize] = n_current_piece;
                    } else {
                        field[(n_current_y + py as i16) as usize]
                            [(n_current_x + px as i16) as usize] = 0;
                    }
                }
            }

            // Draw field
            write!(handle, "{}", clear::All)?;
            for (y, row) in field.iter().enumerate() {
                for (x, &ch) in row.iter().enumerate() {
                    write!(
                        handle,
                        "{}n_current_x = {n_current_x}, n_current_y = {n_current_y}",
                        cursor::Goto(N_FIELD_WIDTH as u16 + 5, 2)
                    )?;
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
