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
    // run "for" command
    pub fn r#for(&self, mut callback: Vec<parser::Token>) -> parser::Token {
        // check argument count
        if callback.len() != 3 {
            debug::send_argc_message("for", 3);
        }

        let do_every_iter = callback.pop().unwrap();
        let variable_name = self.token_to_string(callback.pop().unwrap());
        let will_iterated = self.token_to_string(callback.pop().unwrap());

        // check if can be a int
        let iter_num = will_iterated.parse::<i32>();
        if let Ok(number) = iter_num {
            // iterate over number
            let mut last_output: parser::Token = crate::nil_token!();
            for n in 0..number {
                if !crate::is_false!(variable_name) {
                    self.r#let(vec![
                        parser::Token::String(variable_name.clone()),
                        crate::to_token!(n),
                    ]);
                }

                last_output = self.process(do_every_iter.clone());
            }

            last_output
        } else {
            // iterate over character
            let mut last_output: parser::Token = crate::nil_token!();
            for character in will_iterated.chars() {
                self.r#let(vec![
                    parser::Token::String(variable_name.clone()),
                    crate::to_token!(character),
                ]);

                last_output = self.process(do_every_iter.clone());
            }

            last_output
        }
    }

    // run "while" command
    pub fn r#while(&self, mut callback: Vec<parser::Token>) -> parser::Token {
        // check argument count
        if callback.len() != 2 {
            debug::send_argc_message("while", 2);
        }

        // get arguments
        let do_every_iter = callback.pop().unwrap();
        let checking = callback.pop().unwrap();

        // start loop
        let mut last_output: parser::Token = crate::nil_token!();
        loop {
            if !crate::is_false!(self.token_to_string(checking.clone())) {
                last_output = self.process(do_every_iter.clone());
            } else {
                break;
            }
        }

        last_output
    }
}
