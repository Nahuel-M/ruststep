ruststep
=========

A crate for STEP (Standard for the Exchange of Product model data), standardized as ISO 10303

What is STEP?
--------------

- STEP is a set of data serialize formats, schema language, and common schemas.
- Data serialize format is called **exchange structure**,
  and serialized as ASCII text (ISO-10303-21, usually with extension `*.step`, `*.stp` or `*.p21`)
  or XML (ISO-10303-22).
- Schema language is called **EXPRESS**. EXPRESS file is usually named with extension `*.exp`.
- Many common schemas are defined in ISO-10303 by EXPRESS language.
  - [schemas](./schemas) contains copies
  - Application Protocol (AP) is a class of defined schemas, and the main target of this project.
  - AP203 is most famous one in CAD (computer-aided design) applications.

### Rosetta Stone for web developers

|                 | Protocol Buffers                                           | STEP (ISO 10303)                                         |
|:----------------|:-----------------------------------------------------------|:---------------------------------------------------------|
| Schema Language | [Protocol Buffers Version 3 Language Specification][pbspec]| EXPRESS Language (ISO 10303-11)                          |
| Schema file     | `*.proto` file                                             | `*.exp` file                                             |
| Data            | [Encoded Binary data][pbencoding]                          | "Exchange structure", `*.step` file (ASCII, ISO 10303-21)|
| Compiler        | protoc                                                     | esprc                                                    |

[pbspec]: https://developers.google.com/protocol-buffers/docs/reference/proto3-spec
[pbencoding]: https://developers.google.com/protocol-buffers/docs/encoding

Architecture
-------------

This project consists of following crates:

| name         | description                                                        |
|:-------------|:-------------------------------------------------------------------|
| espr         | [EXPRESS Language (ISO 10303-11)][EXPRESS] Compiler                |
| expr-runtime | Runtime of expr compiler                                           |
| ruststep     | Codes generated by espr compiler, and IO for ASCII and XML formats |

[EXPRESS]: https://www.iso.org/standard/38047.html

See [ARCHITECTURE.md](./ARCHITECTURE.md) for detail.

License
--------
Copyright 2021 RICOS Co. Ltd.

Licensed under the Apache License, Version 2.0 http://www.apache.org/licenses/LICENSE-2.0
