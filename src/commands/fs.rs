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
use std::fs;
use std::path::Path;

impl machine::Machine {
    // run "file.read" command
    pub fn readf(&self, mut callback: Vec<parser::Token>) -> parser::Token {
        // check argument count
        if callback.len() != 1 {
            debug::send_argc_message("file.read", 1);
        }

        // get first argument
        let first_arg = self.token_to_string(callback.pop().unwrap());

        // read file
        let read = fs::read(first_arg);

        match read {
            Ok(content) => crate::to_token!(String::from_utf8_lossy(&content)),
            Err(_) => crate::nil_token!(),
        }
    }

    // run "file.write" command
    pub fn writef(&self, mut callback: Vec<parser::Token>) -> parser::Token {
        // check argument count
        if callback.len() != 2 {
            debug::send_argc_message("file.write", 2);
        }

        // get arguments
        let second_arg = self.token_to_string(callback.pop().unwrap());
        let first_arg = self.token_to_string(callback.pop().unwrap());

        // write file
        crate::to_token!(fs::write(first_arg, second_arg).is_ok())
    }

    // run "file.append" command
    pub fn appendf(&self, mut callback: Vec<parser::Token>) -> parser::Token {
        // check argument count
        if callback.len() != 2 {
            debug::send_argc_message("file.append", 2);
        }

        // get arguments
        let second_arg = self.token_to_string(callback.pop().unwrap());
        let first_arg = self.token_to_string(callback.pop().unwrap());

        // read + write file
        let read = fs::read(&first_arg);

        match read {
            Ok(mut content) => {
                content.extend(second_arg.as_bytes());
                crate::to_token!(fs::write(first_arg, content).is_ok())
            },
            Err(_) => crate::nil_token!()
        }
    }

    // run "file.make" command
    pub fn makef(&self, mut callback: Vec<parser::Token>) -> parser::Token {
        // check argument count
        if callback.len() != 1 {
            debug::send_argc_message("file.make", 1);
        }

        // get arguments
        let first_arg = self.token_to_string(callback.pop().unwrap());

        if !Path::new(&first_arg).exists() {
            crate::to_token!(fs::write(first_arg, "").is_ok())
        } else {
            crate::to_token!(false)
        }
    }

    // run "file.exists" command
    pub fn existsf(&self, mut callback: Vec<parser::Token>) -> parser::Token {
        // check argument count
        if callback.len() != 1 {
            debug::send_argc_message("file.exists", 1);
        }

        // get arguments
        let first_arg = self.token_to_string(callback.pop().unwrap());

        crate::to_token!(Path::new(&first_arg).exists())
    }

    // run "file.remove" command
    pub fn removef(&self, mut callback: Vec<parser::Token>) -> parser::Token {
        // check argument count
        if callback.len() != 1 {
            debug::send_argc_message("file.remove", 1);
        }

        // get arguments
        let first_arg = self.token_to_string(callback.pop().unwrap());

        // remove
        crate::to_token!(fs::remove_file(first_arg).is_ok())
    }

    // run "file.type" command
    pub fn typef(&self, mut callback: Vec<parser::Token>) -> parser::Token {
        // check argument count
        if callback.len() != 1 {
            debug::send_argc_message("file.type", 1);
        }

        // get arguments
        let first_arg = self.token_to_string(callback.pop().unwrap());

        // check if exists
        match fs::metadata(first_arg) {
            Ok(metadata) => {
                if metadata.is_dir() {
                    crate::to_token!("dir")
                } else {
                    crate::to_token!("file")
                }
            }
            Err(_) => crate::nil_token!()
        }
    }
}
