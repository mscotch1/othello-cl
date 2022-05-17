pub enum Command {
    /** Place piece at specified position */
    Move(String),

    /** Display help for using the CLI */
    Help(String),

    /** Exit the software */
    Exit,

    /** Indicate an invalid command */
    Invalid,
}

pub struct CLI {
    history: Vec<String>,
}

impl CLI {
    pub fn new() -> CLI {
        CLI { history: vec![] }
    }

    pub fn prompt(&self) -> Command {
        let mut command = String::new();
        match std::io::stdin().read_line(&mut command) {
            // TODO need to tokenize command
            Ok(_buffer_size) => match command.trim() {
                // "history" => Command::History(0),
                "help" | "?" => Command::Help(String::from("Help!")),
                "exit" => Command::Exit,
                "move" | _ => {
                    // need better logic to tell if actually a move
                    if command.len() != 2 {
                        return Command::Invalid;
                    }
                    Command::Move(command)
                }
            },
            Err(e) => Command::Invalid,
        }
    }
}
