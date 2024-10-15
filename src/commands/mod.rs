use num_traits::SaturatingAdd;
use std::{collections::HashMap, fmt::Display, str::FromStr};

use crate::app::App;
pub mod clear;
pub mod help;
pub mod pos;
pub mod prec;
pub mod quit;
pub mod zoom_factor;

pub struct Command {
    pub execute: &'static dyn Fn(&mut App, Vec<&str>),
    pub name: &'static str,
    pub basic_desc: &'static str,
    pub detailed_desc: Option<&'static str>,
    pub accepted_arg_count: &'static [usize],
}

pub fn create_commands() -> HashMap<&'static str, &'static Command> {
    HashMap::from([
        ("clear", &clear::CLEAR),
        ("quit", &quit::QUIT),
        ("pos", &pos::POS),
        ("help", &help::HELP),
        ("zoom_factor", &zoom_factor::ZOOM_FACTOR),
        ("prec", &prec::PREC),
    ])
}

pub fn command_increment<T>(
    app: &mut App,
    current_val: T,
    args: Vec<&str>,
    min: T,
    max: T,
) -> Option<T>
where
    T: SaturatingAdd + Display + FromStr + PartialOrd,
{
    // If no args are provided, just display the current value
    if args.is_empty() {
        app.log_info(format!(
            "The value is currently set to <acc {}>",
            current_val
        ));
        return None;
    }

    let mut new_val = current_val;

    if args.len() == 1 {
        // If a single argument is provided,
        // parse it and set the value to the provided one
        let parsed = args[0].parse::<T>();
        match parsed {
            Err(_) => {
                app.log_error("The value you provided could not be interpreted.");
                return None;
            }
            Ok(val) => new_val = val,
        }
    } else if args.len() == 2 {
        // If two arguments are given, the first one is the operator
        // and the second one is the increment value
        let operator = args[0];
        let increment = args[1];

        // try to parse the increment value
        let parsed = increment.parse::<T>();
        match parsed {
            Err(_) => {
                app.log_error("The given increment value could not be interpreted.");
                return None;
            }
            Ok(val) => {
                // Check that the operator is either + or -
                new_val = match operator {
                    "+" => new_val.saturating_add(&val),
                    "-" => new_val.saturating_add(&val),
                    _ => {
                        app.log_error("The first argument must be either <acc +> or <acc ->.");
                        return None;
                    }
                }
            }
        }
    }

    if new_val < min || new_val > max {
        app.log_error(format!("The value must stay remain {min} and {max}"));
        return None;
    }

    app.log_success(format!("Value successfully set to <acc {new_val}>."));
    Some(new_val)
}
