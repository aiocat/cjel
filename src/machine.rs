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
use std::mem::take;

// machine struct
pub struct Machine {
    instructions: Vec<parser::Token>, // instructions for machine
}

// implement default for machine
impl Default for Machine {
    // add default function for default trait
    fn default() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }
}

// main part of the machine
impl Machine {
    // add new command to create machine easily
    pub fn new(instructions: Vec<parser::Token>) -> Self {
        let mut machine = Machine::default();
        machine.instructions = instructions;

        machine
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
}
