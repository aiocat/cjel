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
use crate::parser;
use crate::variable;
use std::mem::take;

// machine struct
pub struct Machine {
    instructions: Vec<parser::Token>,       // instructions for machine
    variables: Vec<variable::VariableData>, // variables stored here
}

// implement default for machine
impl Default for Machine {
    // add default function for default trait
    fn default() -> Self {
        Self {
            instructions: Vec::new(),
            variables: Vec::new(),
        }
    }
}

// main part of the machine
impl Machine {
    // add new command to create machine easily
    pub fn new(instructions: Vec<parser::Token>) -> Self {
        Machine {
            instructions,
            ..Default::default()
        }
    }

    // run machine
    pub fn process_whole(&mut self) {
        // iterate over instructions
        for token in take(&mut self.instructions) {
            self.process(token);
        }
    }

    // run a command
    pub fn process(&mut self, token: parser::Token) -> parser::Token {
        // check if its a command
        if let parser::Token::Command(command) = token {
            match command.name.as_str() {
                "print" => self.print(command.arguments),
                "let" => self.r#let(command.arguments),
                "get" => self.get(command.arguments),
                "upgrade" => self.upgrade(command.arguments),
                _ => {
                    // give an error without halting
                    debug::send_message("unknown command");
                    parser::Token::String(String::new())
                }
            }
        } else {
            token
        }
    }

    // run "print" command
    pub fn print(&mut self, callback: Vec<parser::Token>) -> parser::Token {
        let mut arguments: Vec<String> = Vec::new();

        // iterate over given arguments
        for arg in callback {
            // check token type
            if let parser::Token::String(value) = arg {
                // push string
                arguments.push(value);
            } else if let parser::Token::Command(_) = arg {
                // run command and push string
                if let parser::Token::String(value) = self.process(arg) {
                    // push string
                    arguments.push(value);
                }
            }
        }

        // print collected objects
        let calculated_string = arguments.join(" ");
        println!("{calculated_string}");

        parser::Token::String(calculated_string)
    }

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
            .push(variable::VariableData::new(variable_name, variable_value));

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
