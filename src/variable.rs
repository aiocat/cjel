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
use std::rc::Rc;

// variable struct
#[derive(Debug)]
pub struct VariableData {
    pub key: String,       // variable key
    pub value: Rc<String>, // uses rc to share string without memory-cost
    pub ref_count: usize,  // for variable reference counting, will removed if zero
}

// variable functions
impl VariableData {
    // create new variable data
    pub fn new(key: String, value: String) -> Self {
        Self {
            key,
            value: Rc::new(value),
            ref_count: 1,
        }
    }

    // return variable value without cloning and moving, using reference counting
    pub fn get(&mut self) -> String {
        // check reference count
        if self.ref_count == 1 {
            // reset variable value
            let value = take(Rc::get_mut(&mut self.value).unwrap());
            self.ref_count = 0;
            value
        } else if self.ref_count < 1 {
            // return nil
            String::from("nil")
        } else {
            // clone variable reference count
            let mut cloned = Rc::clone(&self.value);
            let value = Rc::make_mut(&mut cloned);
            self.ref_count -= 1;
            take(value)
        }
    }

    // upgrade variable reference count, so variable can live longer
    pub fn upgrade(&mut self) {
        self.ref_count += 1;
    }
}
