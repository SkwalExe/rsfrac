use crate::AppState;
use std::{fmt::Display, str::FromStr};

use num_traits::{SaturatingAdd, SaturatingSub};

pub(crate) fn command_increment<T>(
    state: &mut AppState,
    current_val: T,
    args: Vec<&str>,
    min: T,
    max: T,
) -> Result<T, String>
where
    T: SaturatingAdd + Display + FromStr + PartialOrd + SaturatingSub,
{
    // If no args are provided, just display the current value
    if args.is_empty() {
        state.log_info(format!(
            "The value is currently set to <acc {}>",
            current_val
        ));
        return Ok(current_val);
    }

    let mut new_val = current_val;

    if args.len() == 1 {
        // If a single argument is provided,
        // parse it and set the value to the provided one
        new_val = args[0]
            .parse::<T>()
            .map_err(|_| "The value you provided could not be interpreted.")?;
    } else if args.len() == 2 {
        // If two arguments are given, the first one is the operator
        // and the second one is the increment value
        let operator = args[0];
        let increment = args[1];

        // try to parse the increment value
        let parsed = increment
            .parse::<T>()
            .map_err(|_| "The given increment value could not be interpreted as an integer.")?;
        // Check that the operator is either + or -
        new_val = match operator {
            "+" => new_val.saturating_add(&parsed),
            "-" => new_val.saturating_sub(&parsed),
            _ => {
                return Err("The first argument must be either <acc +> or <acc ->.".to_string());
            }
        }
    }

    if new_val < min || new_val > max {
        return Err(format!("The value must stay remain {min} and {max}"));
    }

    state.log_success(format!("Value successfully set to <acc {new_val}>."));
    Ok(new_val)
}
