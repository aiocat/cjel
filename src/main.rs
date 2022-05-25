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
mod commands;
mod debug;
mod machine;
mod macros;
mod parser;

use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();

    // check command line count
    if args.len() < 2 {
        eprintln!("[JEL] at [READING]: file name must be given.");
        return;
    }

    // get current directory
    let mut path = env::current_dir().unwrap();

    // read file and set directory
    let file_name = &args[1];
    path.push(file_name);

    let file_data = read_to_string(&path).unwrap();
    path.pop();
    let _ = env::set_current_dir(path);

    // run parser
    let mut parser = parser::Parser::new(&file_data);
    parser.parse();

    // dbg!(&parser.output);

    // run interpreter
    let mut machine = machine::Machine::new(parser.output);
    machine.process_whole();
}
