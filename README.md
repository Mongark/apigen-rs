# API Generator

Creates an API based on the specifications in a target JSON file. Communicates with MongoDB.

## TO-DO
-[x] apirc loader
-[ ] schema loader
-[ ] handler factory
-[ ] endpoint factory

## Sample config

```
// File: apirc.json
{
    name: "my-api",
    schemas: [
        "hello_schema.json"
    ],
    routes: [
        {
            uri: "/hello",
            method: "GET",
            schema: "hello_schema"
        }
    ]
}

// File: hello_schema.json
[
    {
        "name": "hello_schema",
        "schema": [
            {
                "name": "hello",
                "type": "string"
            }
        ]
    },
    {
        "name": "world_schema",
        "schema": [
            {
                "name": "world",
                "type": "string"
            }
        ]
    }
]
```

## Commands

`--config or -c` specifies the JSON config file.
