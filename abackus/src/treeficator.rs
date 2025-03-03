#![deny(warnings)]

use crate::ebnf::ParserBuilder;
use earlgrey::{EarleyParser, EarleyForest, Error};
use std::fmt::Debug;

#[derive(Clone,Debug)]
pub enum Sexpr {
    Atom(String),
    List(Vec<Sexpr>),
}

#[derive(Debug,Clone,PartialEq)]
pub enum Tree {
    // 1st element of each option is the matched rule
    // ("[+-]", "+")
    Leaf(String, String),
    // ("E -> E [+-] E", [...])
    Node(String, Vec<Tree>),
}

impl Sexpr {
    pub fn print(&self) -> String {
        let mut out = String::new();
        self.print_helper("", &mut out);
        out
    }

    fn print_helper(&self, level: &str, out: &mut String) {
        match *self {
            Sexpr::Atom(ref lexeme) =>
                *out += &format!("{}`-- {:?}\n", level, lexeme),
            Sexpr::List(ref subn) => {
                *out += &format!("{}`--\n", level);
                if let Some((last, rest)) = subn.split_last() {
                    let l = format!("{}  |", level);
                    for n in rest { n.print_helper(&l, out); }
                    let l = format!("{}   ", level);
                    last.print_helper(&l, out);
                }
            }
        }
    }
}

impl ParserBuilder {
    pub fn treeficator<SI>(self, grammar: &str, start: &str)
        -> impl Fn(SI) -> Result<Vec<Tree>, Error>
        where SI: Iterator, SI::Item: AsRef<str> + Debug
    {
        // User may pre-plug grammar (self.0) with terminals
        // 1. build a parser for user's grammar
        let grammar = ParserBuilder::parse_grammar(self.0, grammar)
            .unwrap_or_else(|e| panic!("treeficator error: {:?}", e))
            .into_grammar(start)
            .unwrap_or_else(|e| panic!("treeficator error: {:?}", e));
        // 2. build evaler that builds trees when executing semantic actions
        let mut tree_builder = EarleyForest::new(
            |sym, tok| Tree::Leaf(sym.to_string(), tok.to_string()));
        for rule in grammar.str_rules() {
            tree_builder.action(&rule.to_string(),
                move |nodes| Tree::Node(rule.to_string(), nodes));
        }
        // 3. make function that parses strings into trees
        let parser = EarleyParser::new(grammar);
        move |tokenizer| tree_builder.eval_all(&parser.parse(tokenizer)?)
    }

    pub fn sexprificator<SI>(self, grammar: &str, start: &str)
        -> impl Fn(SI) -> Result<Vec<Sexpr>, Error>
        where SI: Iterator, SI::Item: AsRef<str> + Debug
    {
        // User may pre-plug grammar (self.0) with terminals
        // 1. build a parser for user's grammar
        let grammar = ParserBuilder::parse_grammar(self.0, grammar)
            .unwrap_or_else(|e| panic!("treeficator error: {:?}", e))
            .into_grammar(start)
            .unwrap_or_else(|e| panic!("treeficator error: {:?}", e));
        // 2. build evaler that builds trees when executing semantic actions
        let mut tree_builder = EarleyForest::new(
            |_, tok| Sexpr::Atom(tok.to_string()));
        for rule in grammar.str_rules() {
            tree_builder.action(&rule, move |mut nodes| match nodes.len() {
                1 => nodes.swap_remove(0),
                _ => Sexpr::List(nodes),
            });
        }
        // 3. make function that parses strings into trees
        let parser = EarleyParser::new(grammar);
        move |tokenizer| tree_builder.eval_all(&parser.parse(tokenizer)?)
    }
}
