use crossterm::ExecutableCommand;

#[derive(Clone, Copy, Debug)]
pub enum TapeElement {
    True,
    False,
    NotSet,
}

impl TapeElement {
    fn char(self) -> char {
        match self {
            Self::False => '0',
            Self::True => '1',
            Self::NotSet => '-'
        }
    }
}

pub struct Tape {
    // [isize::MIN, -1] -> [0, isize::MAX]
    left: Vec<TapeElement>,
    // [0, isize::MAX]
    right: Vec<TapeElement>,

    index: isize,

    view: isize,
}

impl Tape {
    pub fn new(tape: Vec<TapeElement>, starting_index: usize) -> Self {
        let (l, r) = tape.split_at_checked(starting_index).expect("Starting Index was greater than the length of the tape");

        if r.len() == 0 {
            panic!("Starting Index was equal to the length of the tape")
        }

        Self {
            index: 0,
            left: l.iter().map(|a| *a).collect(),
            right: r.iter().map(|a| *a).collect(),
            view: 0,
        }
    } 

    pub fn right(&mut self) {
        self.index += 1;
    }

    pub fn set(&mut self, set: TapeElement) {
        if self.index < 0 {
            let index = (!self.index) as usize;
            if let Some(cell) = self.left.get_mut(index) {
                *cell = set;
            } else {
                self.left.push(set);
            }
        } else {
            if let Some(cell) = self.right.get_mut(self.index as usize) {
                *cell = set;
            } else {
                self.right.push(set);
            }
        }
    }

    pub fn left(&mut self) {
        self.index -= 1;
    }

    pub fn get(&mut self) -> TapeElement {
        if self.index < 0 {
            let index = (!self.index) as usize;
            if let Some(cell) = self.left.get(index) {
                return *cell;
            } else {
                self.left.push(TapeElement::NotSet);
                return TapeElement::NotSet;
            }
        } else {
            if let Some(cell) = self.right.get(self.index as usize) {
                return *cell;
            } else {
                self.right.push(TapeElement::NotSet);
                return TapeElement::NotSet;
            }
        }
    }

    pub const PRINT_LINES: usize = 2;

    /// Returns if it should exit
    pub fn print(&mut self, stdout: &mut std::io::Stdout) -> std::io::Result<bool> {
        while crossterm::event::poll(std::time::Duration::ZERO)? {
            if let crossterm::event::Event::Key(crossterm::event::KeyEvent{code, ..}) = crossterm::event::read()? {
                use crossterm::event::KeyCode;
                match code {
                    KeyCode::Left => self.view -= 1,
                    KeyCode::Right => self.view += 1,
                    KeyCode::Esc => return Ok(true),
                    _ => {},
                }
            }
        }

        stdout.execute(crossterm::cursor::MoveToColumn(0))?;
        
        let width = crossterm::terminal::size()?.0;
        let view = self.view;
        let left = view - ((width >> 1) as isize);
        for index in left..(left + width as isize) {
            print!("{}", self.get_at_index(index).char());
        }

        stdout.execute(crossterm::cursor::MoveDown(1))?;
        stdout.execute(crossterm::cursor::MoveToColumn(0))?;

        for index in left..(left + width as isize) {
            if index == self.index {
                print!("^");
            } else {
                print!(" ");
            }
        }
        
        stdout.execute(crossterm::cursor::MoveUp(1))?;
        stdout.execute(crossterm::cursor::MoveToColumn(width >> 1))?;

        Ok(false)
    }

    fn get_at_index(&self, index: isize) -> TapeElement {
        if index < 0 {
            let index = !index as usize;
            self.left.get(index)
        } else {
            let index = index as usize;
            self.right.get(index)
        }.map(|a| *a).unwrap_or(TapeElement::NotSet)
    }

}