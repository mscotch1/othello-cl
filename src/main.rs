use std::env;

mod display;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut color = false;
    for a in args {
        match a.as_str() {
            "--color" => color = true,
            _ => (),
        }
    }
    
    // process command line arguments
    let display = display::Display {
        color,
    };

    let state: [u8; 64] = [
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 1, 2, 0, 0, 0,
        0, 0, 0, 2, 1, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
    ];

    match display.print(state) {
        Ok(()) => {

        }
        Err(_err) => {

        }
    }
}
