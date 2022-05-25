/**
 * Copyright 2022 aiocat
 * 
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * 
 *     http://www.apache.org/licenses/LICENSE-2.0
 * 
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#include "test.h"

const char* call_from_c(const char* got_arg) {
    int sayi = strtol(got_arg, NULL, 10);
    printf("sonuc: %d\n", sayi + 1);
    return "returned from c";
}

const char* borrow(const char* got_arg) {
    printf("with formatted: \"%s\"\n", got_arg);
    return got_arg;
}