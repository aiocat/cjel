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
use std::io::{stdin, stdout, Write};
use std::thread::sleep;
use std::time::Duration;

impl machine::Machine {
    // run "print" command
    pub fn print(&self, callback: Vec<parser::Token>) -> parser::Token {
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

    // run "input" command
    pub fn input(&self, mut callback: Vec<parser::Token>) -> parser::Token {
        if callback.len() != 1 {
            debug::send_argc_message("input", 1);
        }

        // print message
        print!("{}", self.token_to_string(callback.pop().unwrap()));

        // get input
        let mut input = String::new();
        let _ = stdout().flush();
        stdin()
            .read_line(&mut input)
            .expect("stdio exception in structware.");

        if let Some('\n') = input.chars().next_back() {
            input.pop();
        }

        if let Some('\r') = input.chars().next_back() {
            input.pop();
        }

        crate::to_token!(input)
    }

    // run "sleep" command
    pub fn sleep(&self, mut callback: Vec<parser::Token>) -> parser::Token {
        // check argument count
        if callback.len() != 1 {
            debug::send_argc_message("sleep", 1);
        }

        let first_arg = self.token_to_string(callback.pop().unwrap());

        // sleep
        match first_arg.parse::<u64>() {
            Ok(time) => sleep(Duration::from_millis(time)),
            Err(_) => return crate::nil_token!(),
        }

        crate::nil_token!()
    }
}
