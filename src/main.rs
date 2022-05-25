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
mod parser;

fn main() {
    let mut parser = parser::Parser::new(
        r#"
        dylib(testing E:\Hersey\Acatp\jel\test\ffi\libtest.dll)
        let(result native(testing call_from_c "hello from jel!"))
        let(result2 native(testing borrow "again???"))
        print(get(result) | get(result2))
        "#,
    );
    parser.parse();

    // dbg!(&parser.output);

    let mut machine = machine::Machine::new(parser.output);
    machine.process_whole();
}
