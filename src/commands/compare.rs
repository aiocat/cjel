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
    // run "if" command
    pub fn r#if(&self, mut callback: Vec<parser::Token>) -> parser::Token {
        // check argument count
        if callback.len() != 3 {
            debug::send_argc_message("if", 3);
        }

        // get arguments
        let do_if_false = callback.pop().unwrap();
        let do_if_true = callback.pop().unwrap();
        let condination = self.token_to_string(callback.pop().unwrap());

        if crate::is_false!(&condination) {
            self.process(do_if_false)
        } else {
            self.process(do_if_true)
        }
    }

    // run "equals" command
    pub fn equals(&self, mut callback: Vec<parser::Token>) -> parser::Token {
        // check argument count
        if callback.len() != 2 {
            debug::send_argc_message("equals", 2);
        }

        // get arguments
        let second_object = self.token_to_string(callback.pop().unwrap());
        let first_object = self.token_to_string(callback.pop().unwrap());

        // return if equals
        crate::to_token!(first_object == second_object)
    }

    // run "not" command
    pub fn not(&self, mut callback: Vec<parser::Token>) -> parser::Token {
        // check argument count
        if callback.len() != 1 {
            debug::send_argc_message("not", 1);
        }

        // get arguments
        let got_object = self.token_to_string(callback.pop().unwrap());

        if crate::is_false!(got_object) {
            crate::true_token!()
        } else {
            crate::false_token!()
        }
    }

    // run "bigger" command
    pub fn bigger(&self, mut callback: Vec<parser::Token>) -> parser::Token {
        // check argument count
        if callback.len() != 2 {
            debug::send_argc_message("bigger", 2);
        }

        // get arguments
        let second_object = self.token_to_string(callback.pop().unwrap());
        let first_object = self.token_to_string(callback.pop().unwrap());

        // parse arguments
        let first_number = first_object.parse::<f32>();
        let second_number = second_object.parse::<f32>();

        // check if any errors
        if first_number.is_err() || second_number.is_err() {
            // compare as object
            crate::to_token!(first_object.len() > second_object.len())
        } else {
            // compare as float
            let first_number = first_number.unwrap();
            let second_number = second_number.unwrap();

            crate::to_token!(first_number > second_number)
        }
    }
}
