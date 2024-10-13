//! A library made for easier command line argument management
//!
//! Initially it was supposed to be a boilerplate module for my own command line utilities but it looked pretty easy to make a library
//! You might want to write this yourself for your usecase if my library doesn't help you.
//! 
//! At the moment I haven't tested this library fully, please beware of possible errors even though there shouldn't be any.

use std::env;
// Get functions (getters)
/// Uses the input text and tells you where it is in the args vector, returns an ```Option<usize>```
pub fn get_place(argument: &str) -> Option<usize> {
    let args: Vec<_> = env::args().collect();
    let mut result: Option<usize> = None;

    for index in 0..args.len() {
        if argument.starts_with("-") && !argument.starts_with("--") {
            let mut argument_arr: Vec<_> = argument.chars().collect();
            let mut index_arr: Vec<_> = args[index].chars().collect();
            index_arr.sort();
            argument_arr.sort();
            if argument_arr == index_arr {
                result = Some(index);
                break;
            }
        }
        
        if args[index] == argument {
            result = Some(index);
            break;
        }

    } // no idea what this does, not proud of it, but if it works don't fix it.
    result
}

/// Returns the options ("-argument") in a vector. Returns an empty vector if there is none
pub fn get_options() -> Vec<String> {
    let args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        return vec![];
    }
    let mut result: Vec<String> = vec![];
    for index in 0..args.len() {
        if args[index].starts_with("-") {
            result.push(args[index].clone());
        }
    }
    result
}

/// Returns the specified argument / option's next argument in an ```Option<String>```
pub fn get_next_arg(argument: &str) -> Option<String>{
    let args: Vec<_> = env::args().collect();
    let mut result = None;
    if args.len() == 1 {
        return result;
    }
    let place = get_place(argument);

    if let Some(place) = place { 
        if args.len() == place + 1 {
            return result;
        }
        result = Some(args[place + 1].clone());
    }
    result
}

/// Returns the lone / independent arguments in order. Returns an empty vec if there is none
pub fn get_lones() -> Vec<String> {
    let args: Vec<_> = env::args().collect();
    let mut result = vec![];
    if args.len() == 1 {
        return result;
    }

    for index in 0..args.len() {
        match index {
            0 => continue,
            1 => {
                if !args[1].contains("-") {
                    result.push(args[1].clone());
                }
            }
            _ => {
                if !args[index - 1].contains("-") && !args[index].contains("-") {
                    result.push(args[index].clone());
                }
            } // grrr I HATE nesting.
        }
    }
    result
}

/// Returns ```env::args().collect()``` (the arguments)
pub fn get_full_args() -> Vec<String> {
    env::args().collect()
}

//Boolean return functions (is-ers)
/// Checks if an argument exists
pub fn is_arg(argument: &str) -> bool {
    let place = get_place(argument);
    if let Some(_place) = place {
        return true;
    }
    false
}

