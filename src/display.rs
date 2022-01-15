use std::io;
use colored::*;

pub struct Display {
  pub color: bool,
}

impl Display {
  pub fn print(&self, state: [u8; 64]) -> Result<(), io::Error> {
    let mut index = 0;
    let mut board = String::from("\n");
    let full_rows = 17;
    let full_cols = 17;
    
    let mut row = 8;

    for r in 0..full_rows {
        if r % 2 == 1 {
            board += &format!(" {} ", row);
            row -= 1;
        } else {
            board += "   ";
        }
        for c in 0..full_cols {
            if r == 0 {
                if c == 0 {
                    board += &self.edge("╔");
                    continue;
                }
                if c == full_cols - 1 {
                    board += &self.edge("╗");
                    continue;
                }
                if c % 2 == 0 {
                    board += &self.edge("╦");
                    continue;
                }
            }

            if r == full_rows - 1 {
                if c == 0 {
                    board += &self.edge("╚");
                    continue;
                }
                if c == full_cols - 1 {
                    board += &self.edge("╝");
                    continue;
                }
                if c % 2 == 0 {
                    board += &self.edge("╩");
                    continue;
                }
            }
            
            if r % 2 == 0 {
                if c == 0 {
                    board += &self.edge("╠");
                    continue;
                }
                if c == full_cols - 1 {
                    board += &self.edge("╣");
                    continue;
                }
                if c % 2 == 0 {
                    board += &self.edge("╬");
                    continue;
                }
                board += &self.edge("═══");
                continue;
            }
            
            if c % 2 == 0 {
               board += &self.edge("║");
               continue;
            } 
           
            board += &format!("{}", self.piece(state[index]));
            index += 1;
        }
        board += "\n";
    }
    
    board += "     A   B   C   D   E   F   G   H  ";
    
    println!("{}", board);
    Ok(())
  }
  
  fn edge(&self, s: &str) -> String {
    match self.color {
      true => format!("{}", s.black().on_red()),
      false => String::from(s)
    }
  }
  
  fn piece(&self, piece: u8) -> String {
    match piece {
      1 if self.color => format!("{}", " ■ ".black().on_red()),
      1 if !self.color => String::from(" □ "),
      2 if self.color => format!("{}", " ■ ".white().on_red()),
      2 if !self.color => String::from(" ■ "),
      _ => {
        if self.color {
          return format!("{}", "   ".on_red());
        }
        return String::from("   ");
      },
    }
  }
}
