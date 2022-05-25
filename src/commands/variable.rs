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

use crate::debug;
use crate::machine;
use crate::parser;

// variable struct
#[derive(Debug)]
pub struct VariableData {
    pub value: String, // variable value
    pub public: bool,  // if it is, this data will visible to all .jel files
}

// variable functions
impl VariableData {
    // create new variable data
    pub fn new(value: String) -> Self {
        Self {
            value,
            public: false,
        }
    }

    // take variable
    pub fn take(&mut self) -> String {
        take(&mut self.value)
    }
}

// main part of the command(s)
impl machine::Machine {
    // run "let" command
    pub fn r#let(&self, mut callback: Vec<parser::Token>) -> parser::Token {
        // give error message if argument count is not matching
        if callback.len() != 2 {
            debug::send_argc_message("let", 2);
        }

        // get arguments (reversed)
        let second_arg = callback.pop().unwrap();
        let first_arg = callback.pop().unwrap();

        // get variable name
        let variable_name = self.token_to_string(first_arg);

        // get variable value
        let variable_value = self.token_to_string(second_arg);

        // insert variable
        let mut taken = self.variables.take();
        taken.insert(variable_name, VariableData::new(variable_value));
        self.variables.set(taken);

        parser::Token::String(String::from("nil"))
    }

    // run "pubv" command
    pub fn pubv(&self, mut callback: Vec<parser::Token>) -> parser::Token {
        // give error message if argument count is not matching
        if callback.len() != 1 {
            debug::send_argc_message("pubv", 1);
        }

        // get argument
        let first_arg = callback.pop().unwrap();

        // get variable name
        let variable_name = self.token_to_string(first_arg);

        // toggle visibility
        let mut taken = self.variables.take();
        match taken.get_mut(&variable_name) {
            Some(data) => data.public = !data.public,
            None => debug::send_message(&format!(
                "variable \"{variable_name}\" doesn't exists. (yet?)"
            )),
        }

        self.variables.set(taken);

        // return nil
        parser::Token::String(String::from("nil"))
    }

    // run "get" command
    pub fn get(&self, mut callback: Vec<parser::Token>) -> parser::Token {
        // give error message if argument count is not matching
        if callback.len() != 1 {
            debug::send_argc_message("get", 1);
        }

        // get argument
        let first_arg = callback.pop().unwrap();

        // get variable name
        let variable_name = self.token_to_string(first_arg);

        // dbg!(&self.variables);
        // find variable by key
        let mut taken = self.variables.take();
        let will_return = match taken.get_mut(&variable_name) {
            Some(data) => data.take(),
            None => {
                debug::send_message(&format!(
                    "variable \"{variable_name}\" doesn't exists. (yet?)"
                ));
                String::new()
            }
        };

        // remove variable
        taken.remove(&variable_name).unwrap();

        self.variables.set(taken);
        parser::Token::String(will_return)
    }

    // run "clone" command
    pub fn clone(&self, mut callback: Vec<parser::Token>) -> parser::Token {
        // give error message if argument count is not matching
        if callback.len() != 1 {
            debug::send_argc_message("get", 1);
        }

        // get argument
        let first_arg = callback.pop().unwrap();

        // get variable name
        let variable_name = self.token_to_string(first_arg);

        // dbg!(&self.variables);
        // find variable by key
        let mut taken = self.variables.take();
        let will_return = match taken.get_mut(&variable_name) {
            Some(data) => data.value.clone(),
            None => {
                debug::send_message(&format!(
                    "variable \"{variable_name}\" doesn't exists. (yet?)"
                ));
                String::new()
            }
        };

        self.variables.set(taken);
        parser::Token::String(will_return)
    }
}
