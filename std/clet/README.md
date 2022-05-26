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

# jel/std/clet [unsafe]
Clet is an easy-to-use library for creating variable without giving from performance. Actual library written in C.

- Clet can return a data multiple times, without needing to clone.
- Clet variables are faster (and they have less memory usage) than default variables.
- Clet doesn't guarantee memory safety.

## Unsafe
This library contains unsafe codes. If you don't know what `unsafe` is, You may have a memory problem while using a `unsafe` library. We are trying to minimize this but `C` doesn't guarantee memory safety. `unsafe` codes are mostly for better performance, less memory usage.