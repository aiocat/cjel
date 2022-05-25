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

use crate::commands;
use crate::debug;
use crate::parser;
use std::cell::Cell;
use std::collections::HashMap;
use std::mem::take;

// machine struct
pub struct Machine {
    pub instructions: Vec<parser::Token>, // instructions for machine
    pub variables: Cell<HashMap<String, commands::variable::VariableData>>, // variables are stored here
    pub functions: Cell<HashMap<String, commands::function::FunctionData>>, // functions are stored here
    pub dynamic_libs: Cell<Vec<commands::dylib::DynamicLibraryData>>, // dynamic libraries are stored here
}

// implement default for machine
impl Default for Machine {
    // add default function for default trait
    fn default() -> Self {
        Self {
            instructions: Vec::new(),
            variables: Cell::new(HashMap::new()),
            functions: Cell::new(HashMap::new()),
            dynamic_libs: Cell::new(Vec::new()),
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

    // convert a token to a string
    pub fn token_to_string(&self, token: parser::Token) -> String {
        if let parser::Token::String(value) = token {
            // return string
            value
        } else if matches!(token, parser::Token::Command(_)) {
            // run command and push string
            if let parser::Token::String(value) = self.process(token) {
                // push string
                value
            } else {
                debug::send_message("token must be a valid object.");
                String::new()
            }
        } else {
            debug::send_message("token must be a valid object.");
            String::new()
        }
    }

    // run a command
    pub fn process(&self, token: parser::Token) -> parser::Token {
        // check if its a command
        if let parser::Token::Command(command) = token {
            match command.name.as_str() {
                // from commands/stdio.rs
                "print" => self.print(command.arguments),
                // from commands/variable.rs
                "let" => self.r#let(command.arguments),
                "get" => self.get(command.arguments),
                "clone" => self.clone(command.arguments),
                "pubv" => self.pubv(command.arguments),
                // from commands/function.rs
                "do" => self.r#do(command.arguments),
                "function" => self.function(command.arguments),
                "call" => self.call(command.arguments),
                "pubf" => self.pubf(command.arguments),
                // from commands/dylib.rs
                "dylib" => self.dylib(command.arguments),
                "native" => self.native(command.arguments),
                "pubd" => self.pubd(command.arguments),
                // from commands/import.rs
                "import" => self.import(command.arguments),
                // from commands/compare.rs
                "if" => self.r#if(command.arguments),
                "equals" => self.equals(command.arguments),
                "not" => self.not(command.arguments),
                // empty command is for concat objects
                "" => {
                    let mut arguments: Vec<String> = Vec::new();

                    // iterate over given arguments
                    for arg in command.arguments {
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

                    // return object
                    let calculated_string = arguments.join(" ");
                    parser::Token::String(calculated_string)
                }
                unknown => {
                    // give an error
                    debug::send_message(&format!(
                        "command \"{unknown}\" not found, probably removed or just a misspell?"
                    ));
                    parser::Token::String(String::new())
                }
            }
        } else {
            token
        }
    }
}
