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

// a macro that converts any value to token
#[macro_export]
macro_rules! to_token {
    ($expression:expr) => {
        parser::Token::String(($expression).to_string())
    };
}

// a macro that checks if a object is a false value for comparing
#[macro_export]
macro_rules! is_false {
    ($expression:expr) => {
        $expression == "false"
            || $expression == "0"
            || $expression == "nil"
            || $expression == ""
            || $expression == "0.0"
    };
}

// a macro that returns nil token
#[macro_export]
macro_rules! nil_token {
    () => {
        parser::Token::String(String::from("nil"))
    };
}

// a macro that returns false token
#[macro_export]
macro_rules! false_token {
    () => {
        parser::Token::String(String::from("false"))
    };
}

// a macro that returns true token
#[macro_export]
macro_rules! true_token {
    () => {
        parser::Token::String(String::from("true"))
    };
}

// a macro that returns empty token
#[macro_export]
macro_rules! empty_token {
    () => {
        parser::Token::String(String::new())
    };
}
