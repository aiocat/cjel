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

use crate::debug;
use crate::machine;
use crate::parser;

// function struct
#[derive(Debug, Clone)]
pub struct FunctionData {
    pub key: String,              // function key
    pub arguments: Vec<String>,   // function arguments
    pub value: Rc<parser::Token>, // uses rc to share token without memory-cost
}

// function data functions
impl FunctionData {
    // create new function data
    pub fn new(key: String, value: parser::Token, args: Vec<String>) -> Self {
        Self {
            key,
            arguments: args,
            value: Rc::new(value),
        }
    }

    // return function value
    pub fn get(&mut self) -> parser::Token {
        // clone variable reference count
        let mut cloned = Rc::clone(&self.value);
        let value = Rc::make_mut(&mut cloned);
        take(value)
    }
}

// main part of the functions (with do command)
impl machine::Machine {
    // run "do" command
    pub fn r#do(&mut self, callback: Vec<parser::Token>) -> parser::Token {
        // give error message if argument count is not matching
        if callback.is_empty() {
            debug::send_least_argc_message("do", 1);
        }

        // iterate over commands an run them
        let mut last_output: parser::Token = parser::Token::String(String::from("nil"));
        for arg in callback {
            last_output = self.process(arg);
        }

        last_output
    }

    // run "function" command
    pub fn function(&mut self, mut callback: Vec<parser::Token>) -> parser::Token {
        // give error message if argument count is not matching
        if callback.len() != 3 {
            debug::send_argc_message("function", 3);
        }

        // get arguments (reversed)
        let given_command = callback.pop().unwrap();
        let arguments_token = callback.pop().unwrap();
        let first_arg = callback.pop().unwrap();

        // get function name
        let function_name = self.token_to_string(first_arg);

        // get arguments
        let arguments = if let parser::Token::Command(argument_command) = arguments_token {
            if argument_command.name == String::new() {
                let mut args = Vec::new();

                for arg in argument_command.arguments {
                    if let parser::Token::String(value) = arg {
                        args.push(value);
                    }
                }

                args
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };

        // remove clone if exists
        let mut cloned_value = Rc::clone(&self.functions);
        let mutable_value = Rc::make_mut(&mut cloned_value);
        mutable_value.retain(|var| var.key != function_name);

        // insert function
        mutable_value.push(FunctionData::new(function_name, given_command, arguments));
        self.functions = take(&mut cloned_value);

        // return nil
        parser::Token::String(String::from("nil"))
    }

    // run "call" command
    pub fn call(&mut self, mut callback: Vec<parser::Token>) -> parser::Token {
        // give error message if argument count is not matching
        if callback.is_empty() {
            debug::send_least_argc_message("call", 1);
        }

        // get arguments (reversed)
        callback.reverse();
        let first_arg = callback.pop().unwrap();

        // get function name
        let function_name = self.token_to_string(first_arg);

        // find variable by key
        let mut cloned_value = Rc::clone(&self.functions);
        let mutable_value = Rc::make_mut(&mut cloned_value);
        let found_function = mutable_value
            .iter_mut()
            .find(|var| var.key == function_name);

        // return variable value
        match found_function {
            Some(data) => {
                //get function arguments
                let function_args = data.arguments.iter();

                // check argument count
                if callback.len() != function_args.len() {
                    debug::send_message(&format!(
                        "function \"{}\" excepted {} arguments, got {} argument.",
                        function_name,
                        function_args.len(),
                        callback.len()
                    ));
                    return parser::Token::String(String::new());
                }

                // set variables
                for argument in function_args {
                    self.r#let(vec![
                        parser::Token::String(argument.clone()),
                        callback.pop().unwrap(),
                    ]);
                }

                // call command
                self.process(data.get())
            }
            None => {
                debug::send_message(&format!(
                    "function \"{function_name}\" doesn't exists. (yet?)"
                ));
                parser::Token::String(String::new())
            }
        }
    }
}
