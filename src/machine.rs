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
use std::mem::take;
use std::rc::Rc;

// machine struct
pub struct Machine {
    pub instructions: Vec<parser::Token>, // instructions for machine
    pub variables: Vec<commands::variable::VariableData>, // variables stored here
    pub functions: Rc<Vec<commands::function::FunctionData>>, // functions stored here
}

// implement default for machine
impl Default for Machine {
    // add default function for default trait
    fn default() -> Self {
        Self {
            instructions: Vec::new(),
            variables: Vec::new(),
            functions: Rc::new(Vec::new()),
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
                // from commands/stdio.rs
                "print" => self.print(command.arguments),
                // from commands/variable.rs
                "let" => self.r#let(command.arguments),
                "get" => self.get(command.arguments),
                "upgrade" => self.upgrade(command.arguments),
                // from commands/function.rs
                "do" => self.r#do(command.arguments),
                "function" => self.function(command.arguments),
                "call" => self.call(command.arguments),
                _ => {
                    // give an error
                    debug::send_message("unknown command");
                    parser::Token::String(String::new())
                }
            }
        } else {
            token
        }
    }
}
