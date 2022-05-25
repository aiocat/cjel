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

// this part is extreme unsafe for Rust.
use libloading::Error;
use libloading::{Library, Symbol};
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use std::rc::Rc;

use crate::debug;
use crate::machine;
use crate::parser;

// dylib struct
#[derive(Debug)]
pub struct DynamicLibraryData {
    pub key: String,        // dynamic library key
    pub value: Rc<Library>, // dynamic library data
}

// dynamic library data functions
impl DynamicLibraryData {
    // create new dynamic library data (unsafe code)
    pub fn new(key: String, path: String) -> Self {
        let library = unsafe { Library::new(path) };

        if library.is_err() {
            debug::send_message("dynamic library not found, please check your path!");
        }

        Self {
            key,
            value: Rc::new(library.unwrap()),
        }
    }

    // return raw dynamic library
    pub fn get(&self) -> &Library {
        &self.value
    }

    // call a native function from library
    pub fn call(&self, function: String, arg: String) -> String {
        let self_name = &self.key;
        let library = self.get();
        unsafe {
            // get native function from library
            let native_function: Result<
                Symbol<unsafe extern "C" fn(*const c_char) -> *const c_char>,
                Error,
            > = library.get(function.as_bytes());

            // check if exists
            match native_function {
                Ok(runnable) => {
                    let c_string = CString::new(arg).unwrap();
                    let result = runnable(c_string.as_ptr());

                    CStr::from_ptr(result as *const _)
                        .to_string_lossy()
                        .to_string()
                }
                Err(err) => {
                    debug::send_message(&format!("can't find function \"{function}\" from library \"{self_name}\"\n[INFO] library returned this error: {err}"));
                    String::new()
                }
            }
        }
    }
}

// main part of the dynamic library support for jel
impl machine::Machine {
    // run "dylib" command
    pub fn dylib(&mut self, mut callback: Vec<parser::Token>) -> parser::Token {
        // give error message if argument count is not matching
        if callback.len() != 2 {
            debug::send_argc_message("dylib", 2);
        }

        // get arguments (reversed)
        let second_arg = callback.pop().unwrap();
        let first_arg = callback.pop().unwrap();

        // get library name
        let library_name = if let parser::Token::String(value) = first_arg {
            // return string
            value
        } else if matches!(first_arg, parser::Token::Command(_)) {
            // run command and push string
            if let parser::Token::String(value) = self.process(first_arg) {
                // push string
                value
            } else {
                debug::send_message("library name must be a valid object.");
                String::new()
            }
        } else {
            debug::send_message("library name must be a valid object.");
            String::new()
        };

        // get library path
        let library_path = if let parser::Token::String(value) = second_arg {
            // return string
            value
        } else if matches!(second_arg, parser::Token::Command(_)) {
            // run command and push string
            if let parser::Token::String(value) = self.process(second_arg) {
                // push string
                value
            } else {
                debug::send_message("library path must be a valid object.");
                String::new()
            }
        } else {
            debug::send_message("library path must be a valid object.");
            String::new()
        };

        // remove clone if exists
        self.dynamic_libs.retain(|var| var.key != library_name);

        // insert variable
        self.dynamic_libs
            .push(DynamicLibraryData::new(library_name, library_path));

        parser::Token::String(String::from("nil"))
    }

    // run "native" command
    pub fn native(&mut self, mut callback: Vec<parser::Token>) -> parser::Token {
        // give error message if argument count is not matching
        if callback.len() != 3 {
            debug::send_argc_message("native", 3);
        }

        // get arguments (reversed)
        let third_arg = callback.pop().unwrap();
        let second_arg = callback.pop().unwrap();
        let first_arg = callback.pop().unwrap();

        // get library name
        let library_name = if let parser::Token::String(value) = first_arg {
            // return string
            value
        } else if matches!(first_arg, parser::Token::Command(_)) {
            // run command and push string
            if let parser::Token::String(value) = self.process(first_arg) {
                // push string
                value
            } else {
                debug::send_message("library name must be a valid object.");
                String::new()
            }
        } else {
            debug::send_message("library name must be a valid object.");
            String::new()
        };

        // get function name
        let function_name = if let parser::Token::String(value) = second_arg {
            // return string
            value
        } else if matches!(second_arg, parser::Token::Command(_)) {
            // run command and push string
            if let parser::Token::String(value) = self.process(second_arg) {
                // push string
                value
            } else {
                debug::send_message("library path must be a valid object.");
                String::new()
            }
        } else {
            debug::send_message("library path must be a valid object.");
            String::new()
        };

        // get function arg
        let function_arg = if let parser::Token::String(value) = third_arg {
            // return string
            value
        } else if matches!(third_arg, parser::Token::Command(_)) {
            // run command and push string
            if let parser::Token::String(value) = self.process(third_arg) {
                // push string
                value
            } else {
                debug::send_message("library path must be a valid object.");
                String::new()
            }
        } else {
            debug::send_message("library path must be a valid object.");
            String::new()
        };

        // find library by key
        let library = self.dynamic_libs.iter().find(|var| var.key == library_name);

        match library {
            Some(lib) => parser::Token::String(lib.call(function_name, function_arg)),
            None => {
                debug::send_message(&format!(
                    "dynamic library \"{library_name}\" doesn't exists. (yet?)"
                ));
                parser::Token::String(String::new())
            }
        }
    }
}