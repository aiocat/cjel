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
use std::fs::read_to_string;

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
            variables.insert(key, value);
        }
        self.variables.set(variables);

        // append functions
        let mut functions = self.functions.take();
        for (key, value) in machine.functions.take() {
            functions.insert(key, value);
        }
        self.functions.set(functions);

        // append dynamic libraries
        let mut dynamic_libs = self.dynamic_libs.take();
        for dynamic_lib in machine.dynamic_libs.take() {
            // remove old one, if exists
            dynamic_libs.retain(|var| var.key != dynamic_lib.key);
            dynamic_libs.push(dynamic_lib);
        }
        self.dynamic_libs.set(dynamic_libs);

        parser::Token::String(first_arg)
    }

    // import external file and return results
    fn load_external_file(&self, path: &str) -> machine::Machine {
        // read file
        let file_data = read_to_string(path).unwrap();

        // run parser
        let mut parser = parser::Parser::new(&file_data);
        parser.parse();

        // run interpreter
        let mut machine = machine::Machine::new(parser.output);
        machine.process_whole();

        machine
    }
}
