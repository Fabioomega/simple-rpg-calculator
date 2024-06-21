use once_cell::sync::Lazy;
use regex::Regex;
use std::fmt::format;
use std::fs;
use std::string::String;
use std::vec::Vec;

enum KeywordDef {
    Register,
    Rank,
    Type,
    AlwaysDefense,
    TableAddon,
    RaceMult,
    Start,
    End,
}

#[derive(PartialEq)]
enum CustomIdentifier {
    ORDER,
    CHAOS,
}

#[derive(PartialEq)]
enum ValueType {
    Bool(bool),
    Int(i64),
    Float(f64),
    Custom(CustomIdentifier),
}

enum Token {
    Keyword(KeywordDef),
    Value(ValueType),
    Name(String),
}

#[derive(Clone, Copy)]
pub enum MagicType {
    ORDER,
    CHAOS,
}

#[derive(Clone, Copy)]
pub enum MagicRank {
    Common,
    Uncommon,
    Epic,
    Legendary,
    Mythic,
    Divine,
}

pub struct Magic {
    pub name: String,
    pub rank: MagicRank,
    pub typ: MagicType,
    pub always_def: bool,
    pub table_addon: i64,
    pub race_mult: f64,
}

impl Magic {
    fn new() -> Self {
        Self {
            name: String::new(),
            rank: MagicRank::Common,
            typ: MagicType::ORDER,
            always_def: false,
            table_addon: 0,
            race_mult: 1.0,
        }
    }
}

fn parse_string<'a>(inp: &'a str) -> Vec<&'a str> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s+").unwrap());
    return RE.split(&inp).collect();
}

fn parse_tokens(tokens_to_be: &Vec<&str>) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    for &s in tokens_to_be.iter() {
        match s {
            "register" => tokens.push(Token::Keyword(KeywordDef::Register)),
            "rank" => tokens.push(Token::Keyword(KeywordDef::Rank)),
            "type" => tokens.push(Token::Keyword(KeywordDef::Type)),
            "always_def" => tokens.push(Token::Keyword(KeywordDef::AlwaysDefense)),
            "table_addon" => tokens.push(Token::Keyword(KeywordDef::TableAddon)),
            "race_mult" => tokens.push(Token::Keyword(KeywordDef::RaceMult)),
            "{" => tokens.push(Token::Keyword(KeywordDef::Start)),
            "}" => tokens.push(Token::Keyword(KeywordDef::End)),
            "ORDER" => tokens.push(Token::Value(ValueType::Custom(CustomIdentifier::ORDER))),
            "CHAOS" => tokens.push(Token::Value(ValueType::Custom(CustomIdentifier::CHAOS))),
            "false" => tokens.push(Token::Value(ValueType::Bool(false))),
            "true" => tokens.push(Token::Value(ValueType::Bool(true))),
            _ => match s.parse::<i64>() {
                Ok(value) => tokens.push(Token::Value(ValueType::Int(value))),
                Err(_) => match s.parse::<f64>() {
                    Ok(value) => tokens.push(Token::Value(ValueType::Float(value))),
                    Err(_) => tokens.push(Token::Name(s.to_string())),
                },
            },
        }
    }

    tokens
}

#[inline]
fn is_out_of_bounds(tokens: &Vec<Token>, index: usize) -> bool {
    return tokens.len() < index;
}

#[inline]
fn i_to_magic_rank(i: i64) -> MagicRank {
    match i {
        0 => MagicRank::Common,
        1 => MagicRank::Uncommon,
        2 => MagicRank::Epic,
        3 => MagicRank::Legendary,
        4 => MagicRank::Mythic,
        5 => MagicRank::Divine,
        _ => MagicRank::Common,
    }
}

fn interpret_tokens_to_magic(tokens: &Vec<Token>) -> Result<Vec<Magic>, &'static str> {
    let mut registered_magics: Vec<Magic> = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {
            Token::Keyword(key) => match key {
                KeywordDef::Register => {
                    registered_magics.push(Magic::new());

                    if is_out_of_bounds(tokens, i + 1) {
                        return Err("Out of bounds!");
                    }

                    if let Token::Name(n) = &tokens[i + 1] {
                        let t = registered_magics.len() - 1;
                        registered_magics[t].name = n.clone();
                    } else {
                        return Err("Expected Name but found something else!");
                    }
                    i += 2;
                    continue;
                }
                KeywordDef::Rank => {
                    if registered_magics.is_empty() {
                        return Err("No registered magics!");
                    }

                    if is_out_of_bounds(tokens, i + 1) {
                        return Err("Out of bounds!");
                    }

                    if let Token::Value(v) = &tokens[i + 1] {
                        if let ValueType::Int(vl) = v {
                            let t = registered_magics.len() - 1;
                            registered_magics[t].rank = i_to_magic_rank(*vl);
                        } else {
                            return Err("Expected Int but found something else!");
                        }
                    } else {
                        return Err("Expected Int but found something else!");
                    }
                    i += 2;
                    continue;
                }
                KeywordDef::Type => {
                    if registered_magics.is_empty() {
                        return Err("No registered magics!");
                    }

                    if is_out_of_bounds(tokens, i + 1) {
                        return Err("Out of bounds!");
                    }

                    if let Token::Value(v) = &tokens[i + 1] {
                        if let ValueType::Custom(c) = v {
                            match c {
                                CustomIdentifier::ORDER => {
                                    let t = registered_magics.len() - 1;
                                    registered_magics[t].typ = MagicType::ORDER;
                                }
                                CustomIdentifier::CHAOS => {
                                    let t = registered_magics.len() - 1;
                                    registered_magics[t].typ = MagicType::CHAOS;
                                }
                                _ => {}
                            }
                        } else {
                            return Err("Expected ORDER/CHAOS but found something else!");
                        }
                    } else {
                        return Err("Expected ORDER/CHAOS but found something else!");
                    }
                    i += 2;
                    continue;
                }
                KeywordDef::AlwaysDefense => {
                    if registered_magics.is_empty() {
                        return Err("No registered magics!");
                    }

                    if is_out_of_bounds(tokens, i + 1) {
                        return Err("Out of bounds!");
                    }

                    if let Token::Value(v) = &tokens[i + 1] {
                        if let ValueType::Bool(b) = v {
                            let t = registered_magics.len() - 1;
                            registered_magics[t].always_def = *b;
                        } else {
                            return Err("Expected Bool but found something else!");
                        }
                    } else {
                        return Err("Expected Bool but found something else!");
                    }
                    i += 2;
                    continue;
                }
                KeywordDef::TableAddon => {
                    if registered_magics.is_empty() {
                        return Err("No registered magics!");
                    }

                    if is_out_of_bounds(tokens, i + 1) {
                        return Err("Out of bounds!");
                    }

                    if let Token::Value(v) = &tokens[i + 1] {
                        if let ValueType::Int(vl) = v {
                            let t = registered_magics.len() - 1;
                            registered_magics[t].table_addon = *vl;
                        } else {
                            return Err("Expected Int but found something else!");
                        }
                    } else {
                        return Err("Expected Int but found something else!");
                    }
                    i += 2;
                    continue;
                }
                KeywordDef::RaceMult => {
                    if registered_magics.is_empty() {
                        return Err("No registered magics!");
                    }

                    if is_out_of_bounds(tokens, i + 1) {
                        return Err("Out of bounds!");
                    }

                    if let Token::Value(v) = &tokens[i + 1] {
                        if let ValueType::Float(vl) = v {
                            let t: usize = registered_magics.len() - 1;
                            registered_magics[t].race_mult = *vl;
                        } else {
                            return Err("Expected Int but found something else!");
                        }
                    } else {
                        return Err("Expected Int but found something else!");
                    }
                    i += 2;
                    continue;
                }
                _ => {}
            },
            Token::Name(name) => {}
            Token::Value(value) => {}
        }

        i += 1;
    }

    Ok(registered_magics)
}

fn load_file(filename: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(filename)
}

pub fn process_file_to_magic(filename: &str) -> Result<Vec<Magic>, &'static str> {
    let content = match load_file(filename) {
        Ok(t) => t,
        Err(e) => return Err("The file innit.rpg doesn't exist!"),
    };
    let vec_str = parse_string(&content);
    let tokens = parse_tokens(&vec_str);
    interpret_tokens_to_magic(&tokens)
}
