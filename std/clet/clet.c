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

#include "clet.h"
#define BYTE_BUFFER 1024
#define NEXT_BUFFER sizeof(char) * 1024


map_str_t variables;

const char *clet_init(const char *_)
{
    map_init(&variables);
    return "nil";
}

const char *clet_set(const char *input)
{
    char *key = malloc(BYTE_BUFFER + 1);
    char *value = malloc(BYTE_BUFFER + 1);
    *key = 0;
    *value = 0;

    size_t index = 0;
    size_t value_index = 0;
    unsigned int status = 0;
    while (input[index] != '\0')
    {
        if (status == 0)
        {
            if (input[index] == ' ')
            {
                status = 1;
                index++;
                continue;
            }

            if (index != 0 && index % BYTE_BUFFER == 0)
                key = realloc(key, strlen(key) + NEXT_BUFFER);

            strncat(key, &input[index], 1);
        }
        else
        {
            if (value_index != 0 && value_index % BYTE_BUFFER == 0) {
                value = realloc(value, strlen(value) + NEXT_BUFFER);
            }

            strncat(value, &input[index], 1);
            value_index++;
        }
        index++;
    }

    map_set(&variables, key, value);
    return "nil";
}

const char *clet_get(const char *key)
{
    char **value = map_get(&variables, key);
    if (value)
    {
        return *value;
    }
    else
    {
        return "nil";
    }
}

const char *clet_free(const char *_)
{
    map_deinit(&variables);
    return "nil";
}