<!--
 Copyright 2022 aiocat
 
 Licensed under the Apache License, Version 2.0 (the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at
 
     http://www.apache.org/licenses/LICENSE-2.0
 
 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License.
-->

# Jel
Fast, flexible, memory-safe, easy-to-use, interpreted programming language. _work in progress_

## Example
Hello World:
```julia
print(Hello World!) # this is valid
print("Hello World!") # also valid
print(("" ("" ("" ("" Hello)))) ("" ("" ("" ("" World!))))) # also valid
```

Functions:
```julia
function(is_bigger (first second) bigger(get(first) get(second)))
print(is_bigger(20 5)) # true
```

Simple User Input:
```julia
let(username input("Hello! Whats your name? >>> "))
print(get(username))
```

Dynamic Libraries:
```julia
dylib(test ..\ffi\libtest.dll)
let(result native(test borrow "Hello!")) # prints "with formatted: Hello!"
print(get(result)) # prints "Hello!"
```

Variables
```julia
let(thing 10)
print(get(thing)) # prints 10
print(get(thing)) # error, "thing" is removed.

# in jel, variables are deleted after used. to prevent that, you can clone with clone(variable_name)
# or you can use clet library in /std folder to get faster but unsafe vay to handle variables.
```

## License
Jel is distributed under ALv2 license. for more information:
- https://raw.githubusercontent.com/aiocat/jel/main/LICENSE