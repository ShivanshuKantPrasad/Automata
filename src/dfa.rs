use std::{collections::HashMap, iter::Peekable, str::CharIndices};

pub struct DFA {
    pub states: Vec<String>,                           // Q
    pub alphabet: Vec<String>,                         // E
    pub transition: HashMap<(String, String), String>, // Q * E -> Q
    pub starting_state: String,                        // q0
    pub accepting_states: Vec<String>,                 // F
}

impl TryFrom<String> for DFA {
    type Error = String;
    fn try_from(code: String) -> Result<Self, Self::Error> {
        let mut char_indices = code.char_indices().peekable();
        let dfa = DFA {
            states: states(&mut char_indices)?,
            alphabet: alphabet(&mut char_indices)?,
            starting_state: starting_state(&mut char_indices)?,
            accepting_states: accepting_states(&mut char_indices)?,
            transition: transitions(&mut char_indices)?,
        };

        // Check if starting state is valid
        if !dfa.states.contains(&dfa.starting_state) {
            return Err(format!("{} is not a valid State.", dfa.starting_state));
        }

        let invalid_states = dfa
            .accepting_states
            .iter()
            .fold("".to_string(), |mut err, x| {
                if !dfa.states.contains(x) {
                    err += &format!("Accepting State {} is not a valid state.\n", x);
                }
                err
            });

        if !invalid_states.is_empty() {
            return Err(invalid_states.to_string());
        }

        // Check if all the transitions are valid
        let invalid_transitions =
            dfa.transition
                .iter()
                .fold("".to_string(), |mut err, ((start, alphabet), end)| {
                    let mut error = false;
                    if !dfa.states.contains(start) {
                        err += &format!("Initial State {start},");
                        error = true;
                    }
                    if !dfa.alphabet.contains(alphabet) {
                        err += &format!(" alphabet {alphabet},");
                        error = true;
                    }
                    if !dfa.states.contains(end) {
                        err += &format!(" Final state {end},");
                        error = true;
                    }
                    if error {
                        err += &format!(" in Transition {start},{alphabet} -> {end} are invalid\n");
                    }
                    err
                });

        if !invalid_transitions.is_empty() {
            return Err(invalid_transitions.to_string());
        }

        Ok(dfa)
    }
}

impl Into<String> for DFA {
    fn into(self) -> String {
        let mut parts = Vec::new();

        parts.push(format!("states = [{}]", self.states.join(", ")));
        parts.push(format!("alphabet = [{}]", self.alphabet.join(", ")));
        parts.push(format!("starting_state = {}", self.starting_state));
        parts.push(format!(
            "accepting_states = [{}]",
            self.accepting_states.join(", ")
        ));
        parts.push("transitions =".to_string());

        let transitions = self
            .transition
            .iter()
            .map(|((start, alphabet), end)| format!("    {start},{alphabet} = {end};"))
            .collect::<Vec<_>>()
            .join("\n");

        parts.push(transitions);

        parts.join("\n")
    }
}

fn whitespace(code: &mut Peekable<CharIndices>) {
    while code.next_if(|(_, c)| c.is_whitespace()).is_some() {}
}

fn word(code: &mut Peekable<CharIndices>) -> String {
    whitespace(code);
    let word = std::iter::from_fn(|| {
        code.by_ref()
            .next_if(|(_, ch)| ch.is_alphanumeric() || *ch == '_')
    })
    .map(|(_, c)| c)
    .collect();
    whitespace(code);
    word
}

fn list(code: &mut Peekable<CharIndices>) -> Result<Vec<String>, String> {
    whitespace(code);

    match code.next() {
        Some((_, '[')) => (),
        Some((_, x)) => return Err(format!("Unexpected Symbol '{x}' Expected [")),
        None => return Err("Unexpected End of File.".to_string()),
    };
    let mut list = vec![];
    loop {
        let item = word(code);
        list.push(item);
        match code.next() {
            Some((_, ',')) => (),
            Some((_, ']')) => break,
            Some((_, x)) => return Err(format!("Unexpected Symbol '{x}' Expected ,")),
            None => return Err("Unexpected End of File.".to_string()),
        }
    }
    Ok(list)
}

fn keyword(code: &mut Peekable<CharIndices>, keyword: &str) -> Result<bool, String> {
    match word(code) {
        x if x == keyword => return Ok(true),
        x => Err(format!("Expected {keyword}")),
    }
}

fn char(code: &mut Peekable<CharIndices>, ch: char) -> Result<bool, String> {
    whitespace(code);
    let result = match code.next() {
        Some((_, x)) if x == ch => Ok(true),
        Some((_, x)) => Err(format!("Unexpected Symbol '{x}' Expected {ch}")),
        None => Err("Unexpected End of file.".to_string()),
    };
    whitespace(code);
    return result;
}

fn states(code: &mut Peekable<CharIndices>) -> Result<Vec<String>, String> {
    keyword(code, "states")?;
    char(code, '=')?;
    list(code)
}

fn alphabet(code: &mut Peekable<CharIndices>) -> Result<Vec<String>, String> {
    keyword(code, "alphabet")?;
    char(code, '=')?;
    list(code)
}

fn starting_state(code: &mut Peekable<CharIndices>) -> Result<String, String> {
    keyword(code, "starting_state")?;
    char(code, '=')?;
    Ok(word(code))
}

fn accepting_states(code: &mut Peekable<CharIndices>) -> Result<Vec<String>, String> {
    keyword(code, "accepting_states")?;
    char(code, '=')?;
    list(code)
}

fn transitions(
    code: &mut Peekable<CharIndices>,
) -> Result<HashMap<(String, String), String>, String> {
    keyword(code, "transitions")?;
    char(code, '=')?;

    let mut transitions = HashMap::<(String, String), String>::new();
    while code.peek().is_some() {
        let start_state = word(code);
        match code.next() {
            Some((_, ',')) => (),
            Some((_, ']')) => break,
            Some((_, x)) => return Err(format!("Unexpected Symbol '{x}' Expected ',' or ']'")),
            None => return Err("Unexpected End of File.".to_string()),
        }
        let input = word(code);
        char(code, '=')?;

        let final_state = word(code);
        char(code, ';')?;
        whitespace(code);
        transitions.insert((start_state, input), final_state);
    }
    Ok(transitions)
}

#[cfg(test)]
mod states_tests {
    use super::*;

    #[test]
    fn missing_states_keyword() {
        assert_eq!(
            states(&mut "state = [q1, q2,q3, q4, q5]".char_indices().peekable()),
            Err("Expected states".to_string()),
        );
    }

    #[test]
    fn missing_equal_symbol() {
        assert_eq!(
            states(&mut "states , [q1, q2,q3, q4, q5]".char_indices().peekable()),
            Err("Unexpected Symbol ',' Expected =".to_string()),
        );
    }

    #[test]
    fn valid_parse() {
        assert_eq!(
            states(&mut "states = [q1, q2,q3, q4, q5]".char_indices().peekable()).unwrap(),
            vec!["q1", "q2", "q3", "q4", "q5"]
        );
    }
}

#[cfg(test)]
mod alphabet_tests {
    use super::*;

    #[test]
    fn missing_states_keyword() {
        assert_eq!(
            alphabet(&mut "alphabett = [a,b]".char_indices().peekable()),
            Err("Expected alphabet".to_string()),
        );
    }

    #[test]
    fn missing_equal_symbol() {
        assert_eq!(
            alphabet(&mut "alphabet , [a,b]".char_indices().peekable()),
            Err("Unexpected Symbol ',' Expected =".to_string()),
        );
    }

    #[test]
    fn valid_parse() {
        assert_eq!(
            alphabet(&mut "alphabet = [a,b]".char_indices().peekable()).unwrap(),
            vec!["a", "b"]
        );
    }
}

#[cfg(test)]
mod starting_state_tests {
    use super::*;

    #[test]
    fn missing_states_keyword() {
        assert_eq!(
            starting_state(&mut "starting_statea = q1".char_indices().peekable()),
            Err("Expected starting_state".to_string()),
        );
    }

    #[test]
    fn missing_equal_symbol() {
        assert_eq!(
            starting_state(&mut "starting_state , q1".char_indices().peekable()),
            Err("Unexpected Symbol ',' Expected =".to_string()),
        );
    }

    #[test]
    fn valid_parse() {
        assert_eq!(
            starting_state(&mut "starting_state = q1".char_indices().peekable()).unwrap(),
            "q1"
        );
    }
}

#[cfg(test)]
mod accepting_states_tests {
    use super::*;

    #[test]
    fn missing_states_keyword() {
        assert_eq!(
            accepting_states(
                &mut "accepting_statets = [q1, q2,q3, q4, q5]"
                    .char_indices()
                    .peekable()
            ),
            Err("Expected accepting_states".to_string()),
        );
    }

    #[test]
    fn missing_equal_symbol() {
        assert_eq!(
            accepting_states(
                &mut "accepting_states , [q1, q2,q3, q4, q5]"
                    .char_indices()
                    .peekable()
            ),
            Err("Unexpected Symbol ',' Expected =".to_string()),
        );
    }

    #[test]
    fn valid_parse() {
        assert_eq!(
            accepting_states(
                &mut "accepting_states = [q1, q2,q3, q4, q5]"
                    .char_indices()
                    .peekable()
            )
            .unwrap(),
            vec!["q1", "q2", "q3", "q4", "q5"]
        );
    }
}

#[cfg(test)]
mod transitions_tests {
    use super::*;

    #[test]
    fn valid_parse() {
        let mut tran: HashMap<(String, String), String> = HashMap::new();
        tran.insert(("q1".to_string(), "a".to_string()), "q2".to_string());
        tran.insert(("q1".to_string(), "b".to_string()), "q1".to_string());
        tran.insert(("q2".to_string(), "a".to_string()), "q1".to_string());
        tran.insert(("q2".to_string(), "b".to_string()), "q2".to_string());
        assert_eq!(
            transitions(
                &mut r#"
transitions =
    q1,a = q2;
    q1,b = q1;
    q2,a = q1;
    q2,b = q2;
"#
                .char_indices()
                .peekable()
            )
            .unwrap(),
            tran
        );
    }
}

#[cfg(test)]
mod dfa_io_tests {
    use std::fs;

    use super::*;

    #[test]
    fn io_test() {
        let code = fs::read_to_string("./test.dfa").unwrap();
        let dfa = DFA::try_from(code.clone());
        dfa.expect("Error parsing dfa");
    }
}
