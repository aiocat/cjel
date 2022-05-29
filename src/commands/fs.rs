// Copyright (c) 2022 aiocat
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

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
}
