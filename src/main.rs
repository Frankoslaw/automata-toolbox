#![allow(dead_code)]
use std::{collections::HashMap, collections::VecDeque};

//[a-z] -> a|b|c|d|e.....
fn convert_series_to_unions( input_data: &str ) -> String{
    let mut result: String = String::new();
    let mut bracket_opend: bool = false;
    let ( mut char_from, mut char_to, mut last_char ): ( char, char, char ) = ( ' ', ' ', ' ' );

    for new_char in input_data.chars(){
        if new_char == '\\' || last_char == '\\'{
            result.push(new_char);
            continue;
        }

        if last_char != '[' && !bracket_opend && new_char != '['{
            result.push(new_char);
        }else if last_char == '['{
            char_from = new_char;
            bracket_opend = true;
        }else if last_char == '-'{
            char_to = new_char;
        }else if last_char == ']'{
            bracket_opend = false;
            result.push('(');

            for char_to_append in char_from..((char_to as u8 + 1) as char){
                result.push(char_to_append);
                if char_to_append != char_to{
                    result.push('|');
                }
            }

            result.push(')');
        }

        last_char = new_char;
    }

    return result;
}

fn mark_concatanations( input_data: &str ) -> String{
    let mut result: String = String::new();
    let mut last_char: char = ' ';

    for new_char in input_data.chars(){
        if ![' ', '|', '(', '\\'].contains( &last_char ) && ![')', '*', '|'].contains( &new_char ) {
            result.push('\u{22C5}');
        }

        result.push(new_char);
        last_char = new_char;
    }
    

    return result;
}

//Operations order: *, +, ?, \u{22C5}, |
//Algorithm name: Shunting-Yard Algorithm
fn regex_to_postix( input_data: &str ) -> String{
    let mut result: String = String::new();
    let mut operator_stack: VecDeque<char> = VecDeque::new();
    let mut last_char: char = ' ';
    let operator_precedence: HashMap<char, u32> = HashMap::from([
        ('*', 3),
        ('+', 3),
        ('?', 3),
        ('\u{22C5}', 2),
        ('|', 1),
    ]);

    for new_char in input_data.chars(){
        if new_char == '\\' || last_char == '\\'{
            result.push(new_char);
            continue;
        }

        match new_char{
            '(' => operator_stack.push_back( new_char ),
            ')' => {
                loop{
                    let removed_element = operator_stack.pop_back().unwrap();

                    if removed_element == '('{
                        break;
                    }

                    result.push(removed_element);
                }
            },
            operator if operator_precedence.get( &operator ) != None => {
                loop{
                    match operator_stack.back(){
                        Some(some) => {
                            match operator_precedence.get(&some){
                                Some(some) => {
                                    if some < operator_precedence.get(&operator).unwrap(){
                                        break;
                                    }
                                    
                                    result.push( operator_stack.pop_back().unwrap() );
                                },
                                None => {
                                    break;
                                }
                            }
                        },
                        None => {
                            break;
                        }
                    }
                }

                operator_stack.push_back( new_char );
            },
            other => result.push( other ),
        }

        last_char = new_char;
    }

    for _ in 0..operator_stack.len(){
        result.push( operator_stack.pop_front().unwrap() );
    }

    return result;
}

#[derive(Debug)]
struct Transition{
    exp: char,
    next_node: Box<State>
}

#[derive(Debug)]
struct State{
    is_end: bool,
    transitions: Vec<Transition>,
    epsilon_transitions: Vec<Box<State>>
}

//Add return result with new State: https://doc.rust-lang.org/std/result/
impl State{
    pub fn add_epsilon_transition(&mut self, to: Box<State>) -> (){
        self.epsilon_transitions.push( to );
    }

    pub fn add_transition(&mut self, to: Box<State>, symbol: char) -> (){
        self.transitions.push( Transition{
            exp: symbol,
            next_node: to
        });
    }

    pub fn new( is_end: bool ) -> Self{
        Self {
            is_end,
            transitions: vec![],
            epsilon_transitions: vec![]
        }
    }
}

//Generate epsilon nfa
fn from_epsilon(start: &mut State, end: &State) -> (){
    start.add_epsilon_transition( Box::new(end) );
}

//Generate nfa from char
fn from_symbol() -> (){

}

//Algorithm name: Thompson's construction
fn postix_to_nfa() -> (){

}

fn nfa_to_dfa() -> (){

}

fn dfa_to_min_dfa() -> (){

}

fn merge_dfa() -> (){

}

fn dfa_to_asm() -> (){

}

fn validator() -> (){

}

//https://github.com/Asha20/flou
fn nfa_to_svg() -> (){

}

//https://github.com/Asha20/flou
fn dfa_to_svg() -> (){

}

//TODO:
//Implement unit tests for future
//Publish to github
//Change main.rs to lib.rs
//Document whole code


fn debug_all_functions( input_data: &str ) -> (){
    println!("original regex: {}", input_data);

    let result1: &String = &convert_series_to_unions( input_data );
    println!("series to unions result: {}", result1);

    let result2: &String = &mark_concatanations( result1 );
    println!("mark concatanation: {}", result2);

    let result3: &String = &regex_to_postix( result2 );
    println!("postix result: {}", result3);

    println!("\n\n");
}
fn main(){
    debug_all_functions("[0-9]+(\\.[0-9]+)?");
    debug_all_functions("(\\+|\\-|(\\*(\\*)?)|\\/|%)");
}