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

map_str_t variables;

const char *clet_init(const char *_)
{
    map_init(&variables);
    return "nil";
}

const char *clet_set(const char *input)
{
    struct str* key = str_new();
    struct str* value = str_new();

    if (key == NULL && value == NULL)
    {
        return "nil";
    }
    else if (value == NULL)
    {
        str_free(key);
        return "nil";
    }
    else if (key == NULL)
    {
        str_free(value);
        return "nil";
    }

    size_t index = 0;
    uint8_t status = 0;
    while (input[index] != '\0')
    {
        if (status == 0)
        {
            if (input[index] == ' ')
            {
                status = 1;
                index++;

                if (strcmp(clet_get(key->memory), "nil") != 0)
                    map_remove(&variables, key->memory);

                continue;
            }

            str_push(key, &input[index]);
        }
        else
        {
            str_push(value, &input[index]);
        }
    }

    map_set(&variables, key->memory, value->memory);
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