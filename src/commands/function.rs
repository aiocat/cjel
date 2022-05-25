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

use crate::machine;
use crate::parser;
use crate::debug;

// function struct
#[derive(Debug)]
pub struct FunctionData {
    pub key: String,       // function key
    pub value: Rc<parser::Token>, // uses rc to share token without memory-cost
}

// function data functions
impl FunctionData {
    // create new function data
    pub fn new(key: String, value: parser::Token) -> Self {
        Self {
            key,
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
        if callback.len() < 1 {
            debug::send_least_argc_message("do", 1);
        }

        // iterate over commands an run them
        let mut last_output: parser::Token = parser::Token::String(String::from("nil"));
        for arg in callback {
            last_output = self.process(arg);
        }
        
        last_output
    }
}