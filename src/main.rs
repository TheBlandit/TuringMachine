mod reader;
mod state;
mod tape;

use reader::Reader;
use state::{State, StateContainer, StateTransition};
use tape::{Tape, TapeElement};

// Starting state is the first state
// Ignore lines that start with #
// File in format
// Initial tape
// Tape position
// Starting state
// StateName
// case 0
// case 1
// case _

// An example state is:
// s1
// 0>s4
// 1<s2
// 0>t3
// Where s4, s2 and t3 are defined states
// > means go right and < means go left and 0, 1 means what to write on the current cell

fn main() {
    let mut reader = Reader::new("code.txt");

    let mut tape = {
        let tape_string = reader.next().expect("No Initial tape line");

        let mut tape = Vec::new();

        for char in tape_string.chars() {
            tape.push(match char {
                '0' => TapeElement::False,
                '1' => TapeElement::True,
                '_' => TapeElement::NotSet,
                _ => panic!("Tape contained an illegal character"),
            });
        }

        let read = reader.next().expect("No Starting Position Given");
        let starting_index = read.parse().expect("Starting index was not an integer");

        Tape::new(tape, starting_index)
    };

    let starting_state_name = reader.next().expect("No Starting State");

    let mut state_container = StateContainer::new();
    let mut states = Vec::new();

    while let Some(state_name) = reader.next() {
        let cloned = state_name.clone();
        let state = State::new(&mut reader, state_name, &mut state_container)
            .expect(&format!("Error in creating state: {cloned}"));
        states.push(state);
    }

    let mut current_state_index = state_container.new_used(starting_state_name);

    state_container.verify();

    let mut stdout = std::io::stdout();

    for _ in 0..Tape::PRINT_LINES {
        println!();
    }

    crossterm::terminal::enable_raw_mode().expect("Failed to enable raw mode");

    'run: loop {
        for _ in 0..1 {
            if tape.print(&mut stdout).expect("Io failure") {
                break 'run;
            }

            std::thread::sleep(std::time::Duration::from_millis(1000));
        }

        let cell = tape.get();
        let transition = states[current_state_index].get_transition(cell);

        let StateTransition {
            direction,
            set,
            state,
        } = transition;

        current_state_index = state;
        tape.set(set);

        if direction {
            tape.right();
        } else {
            tape.left();
        }

        if current_state_index == 0 {
            break;
        }
    }

    _ = tape.print(&mut stdout);

    crossterm::terminal::disable_raw_mode().expect("Failed to disable raw mode");

    println!();
    println!();
}
