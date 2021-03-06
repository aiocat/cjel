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

use std::process::exit;

// output debug message and exit
pub fn send(line: usize, column: usize, message: &str) {
    eprintln!("[JEL] at [LINE {line}] [COLUMN {column}]: {message}");
    exit(1);
}

// send debug message without halting
pub fn send_message(message: &str) {
    eprintln!("[JEL]: {message}");
    exit(1);
}

// send debug command argument count message
pub fn send_argc_message(name: &str, count: usize) {
    eprintln!("[JEL]: command \"{name}\" takes {count} argument.");
    exit(1);
}

// send debug command argument count message with least
pub fn send_least_argc_message(name: &str, count: usize) {
    eprintln!("[JEL]: command \"{name}\" takes least {count} argument.");
    exit(1);
}
