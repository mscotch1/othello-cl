use crate::env;

use crate::config;
use crate::display;
use crate::cli;

pub struct Game {
    board_state: [u8; 64],
    config: config::Config,
    display: display::Display,
    cli: cli::CLI,
}

impl Game {
    pub fn new(args: &env::Args) -> Game {
        let arg_vec = env::args().collect();
        let config = config::Config::new(&arg_vec);
        let display = display::Display::new(config.color);
        let cli = cli::CLI::new();
        
        Game {
            board_state: [
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 1, 2, 0, 0, 0,
                0, 0, 0, 2, 1, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
            ],
            config,
            display,
            cli,
        }
    }
    
    pub fn run(&self) {
        self.display.board(&self.board_state);
        self.display.splash();
        self.display.offer_help();
        loop {
            self.display.prompt();
            match self.cli.prompt() {
                cli::Command::Exit => {
                    break;
                }
                cli::Command::Help(help_text) => {
                    println!("{}", help_text);
                }
                cli::Command::Invalid => {
                    println!("Unrecognized command \"\".\nTry 'help' or '?' for more information.");
                }
                _ => {
                    self.display.board(&self.board_state);
                }
            }
        }
    }
}
