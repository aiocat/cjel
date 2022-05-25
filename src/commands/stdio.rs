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

use crate::machine;
use crate::parser;

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
}
