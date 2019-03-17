Very Original Interface Definition
====

void is an interface definition language created as a way to learn the Rust programming language.
The void language allows you to define types and endpoints used to generate code in various languages.
A module is a versioned collection of types, structs and endpoints. An API is a collection of compatible
modules that export said types, objects and endpoints.

#### Progress / Wishlist

- Language definition
    - [ ] Primitives
    - [ ] Modules
    - [ ] Apis / Packages
    - [ ] Comments
    - [ ] Enum
    - [ ] List
    - [ ] Map
    - [ ] Set
    - [ ] HTTP Endpoints
    - [ ] WebSocket Endpoints
    - [ ] Docstrings
    - [ ] Streams
- [ ] Lexer
- [ ] AST
- [ ] Parser
- [ ] Typescript backend
- [ ] Python backend
- [ ] Go backend
- [ ] Caching

#### Language Characteristics

- Typed
- Semicolons
- CamelCase for types, lowercase for primitives
- Whitespace insensitive
- Versioned

#### Examples

```
# Custom structures
type Foo struct {
    bar: int,
    oof: Map<string, int>,
    rab: List<float>,
    raboof: Set<int>,
}

# Type aliases
type UUID string;

# Enums
type Bar enum {
    ONE,
    TWO,
    THREE,
}

# Endpoints
type PerformTransactionResponse struct {};
endpoint PerformTransaction {
    request: struct { userId: int, data: List<string> },
    response: PerformTransactionResponse
}

# Docstrings
/*
General description of type.
modules: A list of module IDs to fetch status for.
*/
type GetStatusRequest struct {
    modules: List<int>
}

/*
General description of endpoint.
*/
endpoint GetStatus {
    request: GetStatusRequest,
    response: GetStatusResponse
}

# Versioned modules
module payment version 3.2;

include customers version 1.2;

type Foo string;

# Versioned APIs. Will expose all APIs defined in the included modules.
api version 1.0;

include customers 1.0;
include batch 1.0;
include payment 1.0;
```
