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
use crate::debug;

// token enum
#[derive(Debug)]
pub enum Token {
    Command(Command), // for commands
    String(String), // for strings
    PlaceHolder(u8) // for placeholders
}

// implement default for token
impl Default for Token {
    // add default function for default trait
    fn default() -> Self {
        Token::PlaceHolder(0xF)
    }
}

// command struct
#[derive(Debug)]
pub struct Command {
    name: String, // command name
    arguments: Vec<Token>, // command arguments
}

// parser struct
#[derive(Debug)]
pub struct Parser<'a> {
    source: &'a str,      // given source
    temp: String, // temporary string to keep collected token
    pub output: Vec<Token>, // parser output

    // these objects for debugging
    line: usize,
    column: usize,
}

// implement default for parser
impl Default for Parser<'_> {
    // add default function for default trait
    fn default() -> Self {
        Self {
            source: "",
            temp: String::new(),
            output: Vec::new(),
            line: 1,
            column: 1,
        }
    }
}

// implement helper functions
impl<'a> Parser<'a> {
    // create new parser
    pub fn new(source: &'a str) -> Self {
        let mut parser = Parser::default();
        parser.source = source;

        parser
    }
}

// main part of the parser combined with lexer
impl Parser<'_> {
    // parse source and append tokens to "output" value
    pub fn parse(&mut self) {
        // iterate over characters
        for character in self.source.chars() {
            self.collect(character);
        }

        // drop source earlier, because we don't need this anymore.
        drop(self.source);
    }

    // parse given character
    fn collect(&mut self, character: char) {
        // match character
        match character {
            '(' => {
                // create new temporary command
                let command_name = self.temp.trim().to_string();
                self.temp = String::new();

                self.output.push(Token::Command(Command{
                    name: command_name,
                    arguments: Vec::new()
                }));

                // push a placeholder
                self.output.push(Token::PlaceHolder(0x0)); 
            },
            ' ' => {
                // move string argument (if exists)
                if !self.temp.is_empty() {
                    self.output.push(Token::String(take(&mut self.temp)));
                }
            }
            ')' => {
                // move string argument (if exists)
                if !self.temp.is_empty() {
                    self.output.push(Token::String(take(&mut self.temp)));
                }

                // get argument until we find the closest command
                let mut args: Vec<Token> = Vec::new();
                println!("{:?}", self.output);

                // get all of the arguments
                loop {
                    if self.output.is_empty() {
                        debug::send(self.line, self.column, "jel thinks you forgot to open a brace.");
                    } else if let Some(Token::PlaceHolder(0x0)) = self.output.last() {
                        self.output.pop();
                        break
                    }

                    args.push(self.output.pop().unwrap());
                }

                // reverse arguments
                args.reverse();

                // get command
                if let Some(Token::Command(command)) = self.output.last_mut() {
                    command.arguments = args;
                } else {
                    debug::send(self.line, self.column, "jel thinks you have a syntax error that he can't even solve.");
                }
            },
            _ => self.temp.push(character)
        }
    }
}