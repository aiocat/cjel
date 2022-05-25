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

use std::mem::take;
use std::rc::Rc;

use crate::machine;
use crate::parser;
use crate::debug;

// variable struct
#[derive(Debug)]
pub struct VariableData {
    pub key: String,       // variable key
    pub value: Rc<String>, // uses rc to share string without memory-cost
    pub ref_count: usize,  // for variable reference counting, will removed if zero
}

// variable functions
impl VariableData {
    // create new variable data
    pub fn new(key: String, value: String) -> Self {
        Self {
            key,
            value: Rc::new(value),
            ref_count: 1,
        }
    }

    // return variable value without cloning and moving, using reference counting
    pub fn get(&mut self) -> String {
        // check reference count
        if self.ref_count == 1 {
            // reset variable value
            let value = take(Rc::get_mut(&mut self.value).unwrap());
            self.ref_count = 0;
            value
        } else if self.ref_count < 1 {
            // return nil
            String::from("nil")
        } else {
            // clone variable reference count
            let mut cloned = Rc::clone(&self.value);
            let value = Rc::make_mut(&mut cloned);
            self.ref_count -= 1;
            take(value)
        }
    }

    // upgrade variable reference count, so variable can live longer
    pub fn upgrade(&mut self) {
        self.ref_count += 1;
    }
}

// main part of the command(s)
impl machine::Machine {
    // run "let" command
    pub fn r#let(&mut self, mut callback: Vec<parser::Token>) -> parser::Token {
        // give error message if argument count is not matching
        if callback.len() != 2 {
            debug::send_argc_message("let", 2);
        }

        // get arguments (reversed)
        let second_arg = callback.pop().unwrap();
        let first_arg = callback.pop().unwrap();

        // get variable name
        let variable_name = if let parser::Token::String(value) = first_arg {
            // return string
            value
        } else if matches!(first_arg, parser::Token::Command(_)) {
            // run command and push string
            if let parser::Token::String(value) = self.process(first_arg) {
                // push string
                value
            } else {
                debug::send_message("variable name must be a valid object.");
                String::new()
            }
        } else {
            debug::send_message("variable name must be a valid object.");
            String::new()
        };

        // get variable value
        let variable_value = if let parser::Token::String(value) = second_arg {
            // return string
            value
        } else if matches!(second_arg, parser::Token::Command(_)) {
            // run command and push string
            if let parser::Token::String(value) = self.process(second_arg) {
                // push string
                value
            } else {
                debug::send_message("variable name must be a valid object.");
                String::new()
            }
        } else {
            debug::send_message("variable name must be a valid object.");
            String::new()
        };

        // remove clone if exists
        self.variables.retain(|var| var.key != variable_name);

        // insert variable
        self.variables
            .push(VariableData::new(variable_name, variable_value));

        parser::Token::String(String::from("nil"))
    }

    // run "get" command
    pub fn get(&mut self, mut callback: Vec<parser::Token>) -> parser::Token {
        // give error message if argument count is not matching
        if callback.len() != 1 {
            debug::send_argc_message("get", 1);
        }

        // get argument
        let first_arg = callback.pop().unwrap();

        // get variable name
        let variable_name = if let parser::Token::String(value) = first_arg {
            // return string
            value
        } else if matches!(first_arg, parser::Token::Command(_)) {
            // run command and push string
            if let parser::Token::String(value) = self.process(first_arg) {
                // push string
                value
            } else {
                debug::send_message("variable name must be a valid object.");
                String::new()
            }
        } else {
            debug::send_message("variable name must be a valid object.");
            String::new()
        };

        // dbg!(&self.variables);
        // find variable by key
        let variable = self
            .variables
            .iter_mut()
            .find(|var| var.key == variable_name);

        // return variable value
        match variable {
            Some(data) => parser::Token::String(data.get()),
            None => {
                debug::send_message(&format!(
                    "variable \"{variable_name}\" doesn't exists. (yet?)"
                ));
                parser::Token::String(String::new())
            }
        }
    }

    // run "upgrade" command
    pub fn upgrade(&mut self, mut callback: Vec<parser::Token>) -> parser::Token {
        // give error message if argument count is not matching
        if callback.len() != 1 {
            debug::send_argc_message("upgrade", 1);
        }

        // get argument
        let first_arg = callback.pop().unwrap();

        // get variable name
        let variable_name = if let parser::Token::String(value) = first_arg {
            // return string
            value
        } else if matches!(first_arg, parser::Token::Command(_)) {
            // run command and push string
            if let parser::Token::String(value) = self.process(first_arg) {
                // push string
                value
            } else {
                debug::send_message("variable name must be a valid object.");
                String::new()
            }
        } else {
            debug::send_message("variable name must be a valid object.");
            String::new()
        };

        // dbg!(&self.variables);
        // find variable by key
        let variable = self
            .variables
            .iter_mut()
            .find(|var| var.key == variable_name);

        // upgrade variable
        match variable {
            Some(data) => {
                data.upgrade();
                parser::Token::String(variable_name)
            }
            None => {
                debug::send_message(&format!(
                    "variable \"{variable_name}\" doesn't exists. (yet?)"
                ));
                parser::Token::String(String::new())
            }
        }
    }
}