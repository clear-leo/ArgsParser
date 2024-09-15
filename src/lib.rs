//! A library made for easier command line argument management
//!
//! Initially it was supposed to be a boilerplate module for my own command line utilities but it looked pretty easy to make a library
//!
//! Usage:
//! - ```get_place(argument)``` returns where in the args the specified argument / option is is, returns 0 if it doesn't exist.
//! - ```get_options()``` returns a vector with the option arguments ("-(option)")
//! - ```get_next_arg(argument)``` returns the next argument of the specified option / argument
//! - ```get_lones()``` returns a vector containing the lone / independent arguments (arguments that are not next to options and not options)
//! - ```get_full_args()``` returns the arguments entirely
//! - ```is_arg(argument)``` returns a bool depending on if the specified argument / option exists.
//!
//! Might wanna write this yourself for your usecase inside a module, this is just a general library that I've made for my OWN usecases.

use std::env;
// Get functions (getters)
/// Uses the input text and tells you where it is in the args vector. Returns 0 if it can't find it.
pub fn get_place(argument: &str) -> usize {
    let args: Vec<_> = env::args().collect();
    let mut result: usize = 0;

    for index in 0..args.len() {
        if argument.starts_with("-") && !argument.starts_with("--") {
            let mut index_arr: Vec<_> = args[index].chars().collect();
            index_arr.sort();
            let mut argument_arr: Vec<_> = argument.chars().collect();
            argument_arr.sort();
            if argument_arr == index_arr {
                result = index;
                break;
            }
        }
        
        if args[index] == argument {
            result = index;
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

/// Returns the specified argument / option's next argument. Returns an empty string if it can't find it.
pub fn get_next_arg(argument: &str) -> String {
    let args: Vec<_> = env::args().collect();
    let mut result = String::new();
    if args.len() == 1 {
        return result;
    }
    let place = get_place(argument);

    if place == 0 {
        return result;
    }

    if args.len() == place + 1 {
        return result;
    }
    result = args[place + 1].clone();
    result
}

/// Returns the lone / independent arguments in order, in a vector. Returns an empty vector if there is none
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
    if get_place(argument) > 0 {
        return true;
    }
    false
}

