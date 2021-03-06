use serde_json::{from_str, Value};

// TODO: move validation schemas to file for flexible management
lazy_static! {
    pub static ref VALIDATION_SCHEMA_CREATE: Value = from_str(
        r#"
        {
            "type": "object",
            "properties": {
                "goods": {
                    "type": "array",
                    "uniqueItems": true,
                    "minItems": 1,
                    "items": {
                        "type": "object",
                        "properties": {
                            "id": {
                                "type": "integer"
                            },
                            "count": {
                                "type": "integer",
                                "minimum": 1
                            }
                        },
                        "required": ["id", "count"],
                        "additionalProperties": false
                    }
                }
            },
            "required": ["goods"],
            "additionalProperties": false
        }"#,
    )
    .unwrap();
}

lazy_static! {
    pub static ref VALIDATION_SCHEMA_UPDATE: Value = from_str(
        r#"
        {
            "type": "object",
            "properties": {
                "goods": {
                    "type": "array",
                    "uniqueItems": true,
                    "minItems": 1,
                    "items": {
                        "type": "object",
                        "properties": {
                            "id": {
                                "type": "integer"
                            },
                            "count": {
                                "type": "integer"
                            },
                            "operation": {
                                "type": "string",
                                "enum": ["update", "delete"]
                            }
                        },
                        "additionalProperties": false,
                        "anyOf": [
                            {
                                "properties": {
                                    "operation": { "const": "delete" }
                                },
                                "required": ["id", "operation"]
                            },
                            {
                                "properties": {
                                    "operation": { "const": "update" }
                                },
                                "required": ["id", "count", "operation"]
                            }
                        ]
                    }
                }
            },
            "required": ["goods"],
            "additionalProperties": false
        }"#,
    )
    .unwrap();
}
