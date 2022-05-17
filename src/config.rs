use std::io;

pub struct Config {
    pub color: bool,
    pub ai_time_limit: u16,
}

impl Config {
    fn process_arg(
        config: &mut Config,
        iter: &mut std::slice::Iter<String>,
        arg: &str
    ) -> Result<(), io::Error> {
        match arg {
            "-h" | "--help" => {
                println!("Help!");
                std::process::exit(0);
            },
            "-c" | "--color" => {
                config.color = true;
            },
            "-t" | "--ai-time" => {
                match iter.next() {
                    Some(time_string) => {
                        config.ai_time_limit = time_string.parse::<u16>().unwrap();
                    },
                    None => {
                        // TODO raise a stink
                        return Result::Err(io::Error::new(io::ErrorKind::Other, "\"--ai-time\" requires an argument"));
                    },
                }
            },
            _ => {
                return Result::Err(io::Error::new(io::ErrorKind::Other, format!("unrecognized option \"{}\"\nTry 'othello-cl --help' for more information.", arg)));
            },
        }
        
        Result::Ok(())
    }

    pub fn new(args: &Vec<String>) -> Config {
        let mut args_iter = args.iter();

        // default config
        let mut config = Config {
            color: false,
            ai_time_limit: 6,
        };

        args_iter.next(); // we don't want name of executable
        
        loop {
            match args_iter.next() {
                Some(a) => {
                    match Config::process_arg(
                        &mut config,
                        &mut args_iter,
                        a.as_str()
                    ) {
                        Ok(_) => {},
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            std::process::exit(1);
                        },
                    }
                }
                None => {
                    break;
                }
            }
        }

        config
    }
}
