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
use std::env;
use std::fs::read_to_string;
use std::path::PathBuf;

impl machine::Machine {
    // run "import" command
    pub fn import(&self, mut callback: Vec<parser::Token>) -> parser::Token {
        if callback.is_empty() {
            debug::send_argc_message("import", 1);
        }

        let first_arg = self.token_to_string(callback.pop().unwrap());
        let machine = self.load_external_file(&first_arg);

        // append variables
        let mut variables = self.variables.take();
        for (key, value) in machine.variables.take() {
            if value.public {
                variables.insert(format!("{first_arg}/{key}"), value);
            }
        }
        self.variables.set(variables);

        // append functions
        let mut functions = self.functions.take();
        for (key, value) in machine.functions.take() {
            if value.public {
                functions.insert(format!("{first_arg}/{key}"), value);
            }
        }
        self.functions.set(functions);

        // append dynamic libraries
        let mut dynamic_libs = self.dynamic_libs.take();
        for mut dynamic_lib in machine.dynamic_libs.take() {
            if dynamic_lib.public {
                // remove old one, if exists
                dynamic_libs.retain(|var| var.key != dynamic_lib.key);

                dynamic_lib.key = format!("{}/{}", first_arg, dynamic_lib.key);
                dynamic_libs.push(dynamic_lib);
            }
        }
        self.dynamic_libs.set(dynamic_libs);

        parser::Token::String(first_arg)
    }

    // import external file and return results
    fn load_external_file(&self, path: &str) -> machine::Machine {
        let old_working_dir = env::current_dir().unwrap();

        // read file
        let file_data = read_to_string(path).unwrap();

        // run parser
        let mut parser = parser::Parser::new(&file_data);
        parser.parse();

        // set working dir
        let mut new_path = PathBuf::from(path);
        new_path.pop();

        let _ = env::set_current_dir(new_path);

        // run interpreter
        let mut machine = machine::Machine::new(parser.output);
        machine.process_whole();

        // re-edit directory
        let _ = env::set_current_dir(old_working_dir);
        machine
    }
}
