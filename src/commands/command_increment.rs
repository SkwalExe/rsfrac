use std::{fmt::Display, str::FromStr};

use num_traits::{SaturatingAdd, SaturatingSub};

use crate::app::AppState;

pub(crate) fn command_increment<T>(
    app_state: &mut AppState,
    current_val: T,
    args: Vec<&str>,
    min: T,
    max: T,
) -> Option<T>
where
    T: SaturatingAdd + Display + FromStr + PartialOrd + SaturatingSub,
{
    // If no args are provided, just display the current value
    if args.is_empty() {
        app_state.log_info(format!(
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
                app_state.log_error("The value you provided could not be interpreted.");
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
                app_state.log_error("The given increment value could not be interpreted.");
                return None;
            }
            Ok(val) => {
                // Check that the operator is either + or -
                new_val = match operator {
                    "+" => new_val.saturating_add(&val),
                    "-" => new_val.saturating_sub(&val),
                    _ => {
                        app_state
                            .log_error("The first argument must be either <acc +> or <acc ->.");
                        return None;
                    }
                }
            }
        }
    }

    if new_val < min || new_val > max {
        app_state.log_error(format!("The value must stay remain {min} and {max}"));
        return None;
    }

    app_state.log_success(format!("Value successfully set to <acc {new_val}>."));
    Some(new_val)
}
