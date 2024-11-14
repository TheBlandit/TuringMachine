use std::collections::HashMap;

use crate::{reader::Reader, tape::TapeElement};

pub struct StateContainer {
    count: usize,
    states: HashMap<String, (bool, usize)>,
}

impl StateContainer {
    pub fn new() -> Self {
        Self {
            count: 0,
            states: HashMap::new(),
        }
    }

    fn new_state(&mut self, name: String) -> usize {
        if let Some(state) = self.states.get_mut(&name) {
            if state.0 {
                panic!("State {name} was defined twice");
            } else {
                state.0 = true;
                return state.1;
            }
        } else {
            let index = self.count;
            self.states.insert(name, (true, index));
            self.count += 1;
            return index;
        }
    }

    pub fn new_used(&mut self, name: String) -> usize {
        if let Some(state) = self.states.get(&name) {
            return state.1;
        } else {
            let index = self.count;
            self.states.insert(name, (false, index));
            self.count += 1;
            return index;
        }
    }

    pub fn verify(self) {
        for state in self.states {
            if !state.1.0 {
                panic!("State {} was not defined", state.0);
            }
        }
    }
}

pub struct State {
    case0: StateTransition,
    case1: StateTransition,
    case_: StateTransition,
}

impl State {
    pub fn new(reader: &mut Reader, state_name: String, state_container: &mut StateContainer) -> Option<Self> {
        state_container.new_state(state_name);

        let case0 = reader.next()?;    
        let case1 = reader.next()?;  
        let case_ = reader.next()?;  

        Some(Self {
            case0: StateTransition::new(&case0, state_container)?,
            case1: StateTransition::new(&case1, state_container)?,
            case_: StateTransition::new(&case_, state_container)?,
        })
    }

    pub fn get_transition(&self, cell: TapeElement) -> StateTransition {
        match cell {
            TapeElement::False => self.case0,
            TapeElement::True => self.case1,
            TapeElement::NotSet => self.case_,
        }
    }
}

#[derive(Clone, Copy)]
pub struct StateTransition {
    /// Right is true
    pub direction: bool,
    /// What to set the current cell to
    pub set: TapeElement,
    /// What state to change to
    pub state: usize,
}

impl StateTransition {
    fn new(line: &str, state_container: &mut StateContainer) -> Option<Self> {
        let mut chars = line.chars();

        let set = match chars.next()? {
            '0' => TapeElement::False,
            '1' => TapeElement::True,
            '_' => TapeElement::NotSet,
            _ => return None,
        };

        let direction = match chars.next()? {
            '>' => true,
            '<' => false,
            _ => return None,
        };

        let state_name = chars.collect::<String>();

        let state = state_container.new_used(state_name);

        Some(Self {
            direction,
            set,
            state,
        })
    }
}