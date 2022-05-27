// Copyright 2022 aiocat
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::debug;
use crate::machine;
use crate::parser;

impl machine::Machine {
    // run "add" command
    pub fn add(&self, mut callback: Vec<parser::Token>) -> parser::Token {
        // check argument count
        if callback.len() != 2 {
            debug::send_argc_message("+", 2);
        }

        // get numbers
        let first_number = self.token_to_string(callback.pop().unwrap());
        let second_number = self.token_to_string(callback.pop().unwrap());

        // convert to float
        let to_float_first = first_number.parse::<f64>();
        let to_float_second = second_number.parse::<f64>();

        // check err
        if to_float_first.is_err() || to_float_second.is_err() {
            debug::send_message("+ command only accepts two number.");
            return crate::nil_token!();
        } else {
            let result = to_float_first.unwrap() + to_float_second.unwrap();

            if result.fract() == 0.0 {
                crate::to_token!(result as isize)
            } else {
                crate::to_token!(result)
            }
        }
    }

    // run "sub" command
    pub fn sub(&self, mut callback: Vec<parser::Token>) -> parser::Token {
        // check argument count
        if callback.len() != 2 {
            debug::send_argc_message("-", 2);
        }

        // get numbers
        let first_number = self.token_to_string(callback.pop().unwrap());
        let second_number = self.token_to_string(callback.pop().unwrap());

        // convert to float
        let to_float_first = first_number.parse::<f64>();
        let to_float_second = second_number.parse::<f64>();

        // check err
        if to_float_first.is_err() || to_float_second.is_err() {
            debug::send_message("- command only accepts two number.");
            return crate::nil_token!();
        } else {
            let result = to_float_first.unwrap() - to_float_second.unwrap();

            if result.fract() == 0.0 {
                crate::to_token!(result as isize)
            } else {
                crate::to_token!(result)
            }
        }
    }

    // run "mul" command
    pub fn mul(&self, mut callback: Vec<parser::Token>) -> parser::Token {
        // check argument count
        if callback.len() != 2 {
            debug::send_argc_message("*", 2);
        }

        // get numbers
        let first_number = self.token_to_string(callback.pop().unwrap());
        let second_number = self.token_to_string(callback.pop().unwrap());

        // convert to float
        let to_float_first = first_number.parse::<f64>();
        let to_float_second = second_number.parse::<f64>();

        // check err
        if to_float_first.is_err() || to_float_second.is_err() {
            debug::send_message("* command only accepts two number.");
            return crate::nil_token!();
        } else {
            let result = to_float_first.unwrap() * to_float_second.unwrap();

            if result.fract() == 0.0 {
                crate::to_token!(result as isize)
            } else {
                crate::to_token!(result)
            }
        }
    }

    // run "div" command
    pub fn div(&self, mut callback: Vec<parser::Token>) -> parser::Token {
        // check argument count
        if callback.len() != 2 {
            debug::send_argc_message("/", 2);
        }

        // get numbers
        let first_number = self.token_to_string(callback.pop().unwrap());
        let second_number = self.token_to_string(callback.pop().unwrap());

        // convert to float
        let to_float_first = first_number.parse::<f64>();
        let to_float_second = second_number.parse::<f64>();

        // check err
        if to_float_first.is_err() || to_float_second.is_err() {
            debug::send_message("/ command only accepts two number.");
            crate::nil_token!()
        } else {
            // check if zero
            let second = to_float_second.unwrap();
            if second == 0.0 {
                debug::send_message("[command /]: second number can't be zero (0).");
                return crate::nil_token!();
            }

            let result = to_float_first.unwrap() / second;
            if result.fract() == 0.0 {
                crate::to_token!(result as isize)
            } else {
                crate::to_token!(result)
            }
        }
    }
    // run "mod" command
    pub fn r#mod(&self, mut callback: Vec<parser::Token>) -> parser::Token {
        // check argument count
        if callback.len() != 2 {
            debug::send_argc_message("%", 2);
        }

        // get numbers
        let first_number = self.token_to_string(callback.pop().unwrap());
        let second_number = self.token_to_string(callback.pop().unwrap());

        // convert to float
        let to_float_first = first_number.parse::<f64>();
        let to_float_second = second_number.parse::<f64>();

        // check err
        if to_float_first.is_err() || to_float_second.is_err() {
            debug::send_message("% command only accepts two number.");
            return crate::nil_token!();
        } else {
            let result = to_float_first.unwrap() % to_float_second.unwrap();

            if result.fract() == 0.0 {
                crate::to_token!(result as isize)
            } else {
                crate::to_token!(result)
            }
        }
    }
}
