use crate::dfa::DFA;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Graph {
    nodes: Vec<String>,
    adj_mat: HashMap<(String, String), Vec<String>>,
}

impl From<DFA> for Graph {
    fn from(dfa: DFA) -> Graph {
        let mut adj_mat: HashMap<(String, String), Vec<String>> = HashMap::new();
        dfa.transition.iter().for_each(|((start, alphabet), end)| {
            adj_mat
                .entry((start.to_string(), end.to_string()))
                .or_default()
                .push(alphabet.to_string())
        });
        Graph {
            nodes: dfa.states,
            adj_mat,
        }
    }
}

#[cfg(test)]
mod graph_tests {
    use std::{collections::HashMap, fs};

    use crate::{dfa::DFA, graph::Graph};

    #[test]
    fn dfa_to_graph() {
        let dfa_txt = fs::read_to_string("test.dfa").unwrap();
        assert_eq!(
            Graph::from(DFA::try_from(dfa_txt).unwrap()),
            Graph {
                nodes: vec![String::from("q1"), String::from("q2")],
                adj_mat: HashMap::from([
                    (
                        ("q2".to_string(), "q2".to_string()),
                        vec!["a".to_string(), "b".to_string()]
                    ),
                    (("q1".to_string(), "q1".to_string()), vec!["b".to_string()]),
                    (("q1".to_string(), "q2".to_string()), vec!["a".to_string()])
                ])
            },
        );
    }
}
