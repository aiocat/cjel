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
use std::mem::take;

// parser state enum
#[derive(Debug)]
pub enum ParserState {
    Token,
    String,
    Comment,
}

// token enum
#[derive(Debug, Clone)]
pub enum Token {
    Command(Command), // for commands
    String(String),   // for strings
    PlaceHolder(u8),  // for placeholders
}

// implement default for token
impl Default for Token {
    // add default function for default trait
    fn default() -> Self {
        Token::PlaceHolder(0xF)
    }
}

// command struct
#[derive(Debug, Clone, Default)]
pub struct Command {
    pub name: String,          // command name
    pub arguments: Vec<Token>, // command arguments
}

// parser struct
#[derive(Debug)]
pub struct Parser<'a> {
    // these objects for debugging
    line: usize,
    column: usize,

    source: &'a str,        // given source
    temp: String,           // temporary string to keep collected token
    pub output: Vec<Token>, // parser output
    state: ParserState,     // parser state (normal or string collecting)
}

// implement default for parser
impl Default for Parser<'_> {
    // add default function for default trait
    fn default() -> Self {
        Self {
            source: "",
            temp: String::new(),
            output: Vec::new(),
            state: ParserState::Token,
            line: 1,
            column: 0,
        }
    }
}

// implement helper functions
impl<'a> Parser<'a> {
    // create new parser
    pub fn new(source: &'a str) -> Self {
        Parser {
            source,
            ..Default::default()
        }
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

        // clear garbage data
        self.clear_garbage();
    }

    // parse given character
    fn collect(&mut self, character: char) {
        // match character
        match self.state {
            ParserState::Token => self.collect_token(character),
            ParserState::String => self.collect_string(character),
            ParserState::Comment => self.collect_comment(character),
        }
    }

    // collect normal token
    fn collect_token(&mut self, character: char) {
        match character {
            '(' => {
                // create new temporary command
                let command_name = self.temp.trim().to_string();
                self.temp = String::new();

                self.output.push(Token::Command(Command {
                    name: command_name,
                    ..Default::default()
                }));

                // push a placeholder
                self.output.push(Token::PlaceHolder(0x0));
            }
            '\r' => return,
            '\n' => {
                // move string argument (if exists)
                if !self.temp.is_empty() {
                    self.output.push(Token::String(take(&mut self.temp)));
                }

                // update column and line
                self.line += 1;
                self.column = 0;
            }
            ' ' => {
                // move string argument (if exists)
                if !self.temp.is_empty() {
                    self.output.push(Token::String(take(&mut self.temp)));
                }
            }
            '"' => {
                // move string argument (if exists)
                if !self.temp.is_empty() {
                    self.output.push(Token::String(take(&mut self.temp)));
                }

                // change state to string collecting
                self.state = ParserState::String
            }
            '#' => {
                // move string argument (if exists)
                if !self.temp.is_empty() {
                    self.output.push(Token::String(take(&mut self.temp)));
                }

                // change state to comment collecting
                self.state = ParserState::Comment
            }
            ')' => {
                // move string argument (if exists)
                if !self.temp.is_empty() {
                    self.output.push(Token::String(take(&mut self.temp)));
                }

                // get argument until we find the closest command
                let mut args: Vec<Token> = Vec::new();

                // get all of the arguments
                loop {
                    if self.output.is_empty() {
                        debug::send(
                            self.line,
                            self.column,
                            "jel thinks you forgot to open a brace.",
                        );
                    } else if let Some(Token::PlaceHolder(0x0)) = self.output.last() {
                        self.output.pop();
                        break;
                    }

                    args.push(self.output.pop().unwrap());
                }

                // reverse arguments
                args.reverse();

                // set arguments
                if let Some(Token::Command(command)) = self.output.last_mut() {
                    command.arguments = args;
                } else {
                    debug::send(
                        self.line,
                        self.column,
                        "jel thinks you have a syntax error that he can't even solve.",
                    );
                }
            }
            _ => {
                // push character to temp value
                self.temp.push(character)
            }
        }

        self.column += 1;
    }

    // collect string
    fn collect_string(&mut self, character: char) {
        // check string close
        if character == '"' {
            // check if has an escape
            if !self.temp.is_empty() {
                let last_character = self.temp.chars().last().unwrap();

                if last_character != '\\' {
                    self.state = ParserState::Token;
                    self.output.push(Token::String(take(&mut self.temp)));
                }
            } else {
                self.state = ParserState::Token;
                self.output.push(Token::String(take(&mut self.temp)));
            }
        } else if character == 'n' {
            // check if is a new line
            if !self.temp.is_empty() {
                let last_character = self.temp.chars().last().unwrap();

                if last_character == '\\' {
                    self.temp.pop();
                    self.temp.push('\n');
                    return;
                }
            }

            // push character
            self.temp.push(character);
        } else if character == '\n' {
            // end string if new line
            self.line += 1;
            self.column = 0;

            self.state = ParserState::Token;
            self.output.push(Token::String(take(&mut self.temp)));
        } else {
            // push character
            self.temp.push(character);
        }
    }

    // collect comment
    fn collect_comment(&mut self, character: char) {
        // check new line
        if character == '\n' {
            self.line += 1;
            self.column = 0;
            self.state = ParserState::Token;
        }
    }

    // clean garbage data
    fn clear_garbage(&mut self) {
        self.output
            .retain(|value| matches!(value, Token::Command(_)));
    }
}
