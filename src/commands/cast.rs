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
    // run "float" command
    pub fn float(&self, mut callback: Vec<parser::Token>) -> parser::Token {
        // check argument count
        if callback.len() != 1 {
            debug::send_argc_message("float", 1);
        }

        let will_converted = self.token_to_string(callback.pop().unwrap());
        let will_converted = will_converted.as_str();

        // cast to float
        match will_converted {
            "true" => crate::to_token!(1.0),
            "false" => crate::to_token!(0.0),
            "nil" => crate::to_token!(0.0),
            _ => {
                // try cast to float
                let try_float = will_converted.parse::<f32>();
                match try_float {
                    Ok(value) => crate::to_token!(value as f32),
                    Err(_) => {
                        // try cast to int
                        let try_int = will_converted.parse::<i32>();
                        match try_int {
                            Ok(value) => crate::to_token!(value),
                            Err(_) => crate::nil_token!(),
                        }
                    }
                }
            }
        }
    }

    // run "int" command
    pub fn int(&self, mut callback: Vec<parser::Token>) -> parser::Token {
        // check argument count
        if callback.len() != 1 {
            debug::send_argc_message("int", 1);
        }

        let will_converted = self.token_to_string(callback.pop().unwrap());
        let will_converted = will_converted.as_str();

        // cast to integer
        match will_converted {
            "true" => crate::to_token!(1),
            "false" => crate::to_token!(0),
            "nil" => crate::to_token!(0),
            _ => {
                // try cast to float
                let try_float = will_converted.parse::<f32>();
                match try_float {
                    Ok(value) => crate::to_token!(value as i32),
                    Err(_) => {
                        // try cast to int
                        let try_int = will_converted.parse::<i32>();
                        match try_int {
                            Ok(value) => crate::to_token!(value),
                            Err(_) => crate::nil_token!(),
                        }
                    }
                }
            }
        }
    }

    // run "bool" command
    pub fn bool(&self, mut callback: Vec<parser::Token>) -> parser::Token {
        // check argument count
        if callback.len() != 1 {
            debug::send_argc_message("int", 1);
        }

        let will_converted = self.token_to_string(callback.pop().unwrap());
        let will_converted = will_converted.as_str();

        // cast to integer
        match will_converted {
            "true" => crate::to_token!(true),
            "false" => crate::to_token!(false),
            "nil" => crate::to_token!(false),
            _ => {
                // try cast to float
                let try_float = will_converted.parse::<f32>();
                match try_float {
                    Ok(value) => crate::to_token!(value != 0.0),
                    Err(_) => {
                        // try cast to int
                        let try_int = will_converted.parse::<i32>();
                        match try_int {
                            Ok(value) => crate::to_token!(value != 0),
                            Err(_) => crate::nil_token!(),
                        }
                    }
                }
            }
        }
    }
}
