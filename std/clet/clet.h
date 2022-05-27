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

#ifndef CLET_H
#define CLET_H

#include "./map/src/map.c"
#include <stdio.h>
#include <string.h>

const char* clet_init(const char* _);
const char* clet_set(const char* input);
const char* clet_get(const char* key);
const char* clet_free(const char* _);

#endif /* CLET_H */
