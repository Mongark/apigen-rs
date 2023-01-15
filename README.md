# API Generator

Creates an API based on the specifications in a target JSON file. Communicates with MongoDB.

## Data Types

```typescript
interface EndpointSpec<T> {
    uri: string,
    type: string,       // GET, POST, UPDATE, etc.
    model: Model<T>,
}
```

## Bussiness Rules

Endpoint Generator
- [ ] `Endpoint` should be created from a `EndpointSpec` interface.
- [ ] `Endpoint.send(data)` should run the `Endpoint` callback function.

## General TO-DO
- [x] apirc loader
- [ ] schema loader
- [ ] handler factory
- [ ] endpoint factory
- [ ] implement the config argument

## Sample config

apirc.json
```json
{
    "name": "my-api",
    "schemas": [
        "hello_schema.json"
    ],
    "routes": [
        {
            "uri": "/hello",
            "method": "GET",
            "schema": "hello_schema"
        }
    ]
},
```

hello_schema.json
```json
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
