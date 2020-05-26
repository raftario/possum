# Specification

Since `possum` only targets WebAssembly, you can refer to the [WebAssembly specification](https://webassembly.github.io/spec/core/) for more details.

Syntax definitions use [EBNF](https://www.w3.org/TR/REC-xml/#sec-notation) notation.

## Table of Contents

- [Specification](#specification)
  - [Table of Contents](#table-of-contents)
  - [Paradigm](#paradigm)
  - [Primitives](#primitives)
    - [Integers](#integers)
      - [Syntax](#syntax)
        - [Literal](#literal)
    - [Floating point numbers](#floating-point-numbers)
      - [Syntax](#syntax-1)
        - [Literal](#literal-1)
    - [Booleans](#booleans)
      - [Syntax](#syntax-2)
    - [Tuples](#tuples)
      - [Syntax](#syntax-3)
        - [Type](#type)
        - [Literal](#literal-2)
    - [Slices](#slices)
      - [Syntax](#syntax-4)
        - [Type](#type-1)
    - [Arrays](#arrays)
      - [Syntax](#syntax-5)
        - [Type](#type-2)
        - [Literal](#literal-3)
    - [Reference](#reference)
      - [Syntax](#syntax-6)
        - [Type](#type-3)
    - [Pointers](#pointers)
      - [Syntax](#syntax-7)
        - [Type](#type-4)
    - [Function pointers](#function-pointers)
      - [Syntax](#syntax-8)
        - [Type](#type-5)
  - [Definitions](#definitions)
    - [Functions](#functions)
    - [Structs](#structs)
    - [Enums](#enums)
  - [Function calls](#function-calls)
  - [Casts](#casts)
    - [Integers](#integers-1)
    - [Others](#others)
  - [Operators](#operators)
    - [Postfix](#postfix)
    - [Unary](#unary)
    - [Literal](#literal-4)
      - [Unary](#unary-1)
      - [Binary](#binary)
  - [Syntax reference](#syntax-reference)

## Paradigm

`possum` is an expression based, strictly typed, pure functional language. Everything is an expression, data is immutable and the only control flow constructs are functions, pattern matchers and blocks.

## Primitives

### Integers

| Identifier | Stack | Heap  | Description              |
| ---------- | ----- | ----- | ------------------------ |
| `s8`       | `i32` | `i8`  | 8 bits signed integer    |
| `s16`      | `i32` | `i16` | 16 bits signed integer   |
| `s32`      | `i32` | `i32` | 32 bits signed integer   |
| `s64`      | `i64` | `i64` | 64 bits signed integer   |
| `u8`       | `i32` | `i8`  | 8 bits unsigned integer  |
| `u16`      | `i32` | `i16` | 16 bits unsigned integer |
| `u32`      | `i32` | `i32` | 32 bits unsigned integer |
| `u64`      | `i64` | `i64` | 64 bits unsigned integer |

#### Syntax

##### Literal

```ebnf
Int ::= Sign? ( Digit+ |
                HexInt |
                OctInt |
                BinInt )

HexInt   ::= 0 [xX] HexDigit+
OctInt   ::= 0 [oO] OctDigit+
BinInt   ::= 0 [bB] BinDigit+
BinDigit ::= [01]
```

-   `123`
-   `0xFF`
-   `0o007`
-   `0b0101`

[_ref_](#syntax-reference)

### Floating point numbers

| Identifier | Description            |
| ---------- | ---------------------- |
| `f32`      | 8 bits floating point  |
| `f64`      | 16 bits floating point |

#### Syntax

##### Literal

```ebnf
Float ::= Sign? ( Number | 'inf' | 'NaN' )

Number ::= Digit+ '.' Digit+ Exp?
Exp    ::= [eE] Sign? Digit+
```

-   `3.5`
-   `2.1e-12`

[_ref_](#syntax-reference)

### Booleans

Internally, `false` is `0` and `true` is `1`

| Identifier | Stack | Heap |
| ---------- | ----- | ---- |
| `bool`     | `i32` | `i8` |

#### Syntax

```ebnf
Bool ::= 'false' | 'true'
```

### Tuples

Tuples are unnamed heterogeneous groups of types

#### Syntax

##### Type

```ebnf
TupleT ::= '(' WhiteSpace*
           ( Type CommaSep )*
           Type?
           WhiteSpace* ')'
```

-   `()`
-   `(T)`
-   `(T, U)`

[_ref_](#syntax-reference)

##### Literal

```ebnf
TupleL ::= '(' WhiteSpace*
           ( Expr CommaSep )*
           Expr?
           WhiteSpace* ')'
```

-   `()`
-   `(0, false)`

[_ref_](#syntax-reference)

### Slices

Slices are a view on a sequence of elements of the same type in memory

#### Syntax

##### Type

```ebnf
SliceT ::= '[' WhiteSpace* Type WhiteSpace* ']'
```

-   `[T]`

[_ref_](#syntax-reference)

### Arrays

Arrays are fixed size sequences of elements of the same type

#### Syntax

##### Type

```ebnf
ArrayT ::= '[' WhiteSpace*
           Type
           WhiteSpace* ';' WhiteSpace*
           Int
           WhiteSpace* ']'
```

-   `[T; 8]`
-   `[U; 0xFF]`

[_ref_](#syntax-reference)

##### Literal

```ebnf
ArrayL ::= '[' WhiteSpace*
           ( Expr CommaSep )*
           Expr?
           WhiteSpace* ']'
```

-   `[0, 1, 2, 3]`
-   `[false, true]`

[_ref_](#syntax-reference)

### Reference

#### Syntax

##### Type

```ebnf
Ref ::= '&' Lifetime? Type
```

-   `&T`
-   `&'a T`

[_ref_](#syntax-reference)

### Pointers

#### Syntax

##### Type

```ebnf
Ptr ::= '*' Type
```

-   `*T`

[_ref_](#syntax-reference)

### Function pointers

#### Syntax

##### Type

```ebnf
Fn ::= Specifiers? WhiteSpace* TupleT WhiteSpace* '->' WhiteSpace* Type
```

-   `() -> ()`
-   `<'a, T>(&'a [T]) -> &'a T`

## Definitions

### Functions

TODO

### Structs

TODO

### Enums

TODO

## Function calls

Not having methods makes it possible to use shorcuts for function calls and avoid nesting. Arguments are implicitely casted, and they can precede the function call as a tuple.

-   `a1.function(b, c)` is a valid way to write `function(a2, b, c)` if `a1` can be casted to `a2` (`a1` will first be casted to `(a1)`)
-   `(a1, b2).function(c)` is a valid way to write `function(a2, b2, c)` if `a1` can be casted to `a2` and `b1` can be casted to `b2`

## Casts

Casts can be implicit or explicit using the `as` keyword

### Integers

| `as`  | `s8` | `s16` | `s32` | `s64` | `u8` | `u16` | `u32` | `u64` | `bool` |
| ----- | :--: | :---: | :---: | :---: | :--: | :---: | :---: | :---: | :----: |
| `s8`  |  X   |       |       |       |      |       |       |       |   X    |
| `s16` |  X   |   X   |       |       |  X   |       |       |       |   X    |
| `s32` |  X   |   X   |   X   |       |  X   |   X   |       |       |   X    |
| `s64` |  X   |   X   |   X   |   X   |  X   |   X   |   X   |       |   X    |
| `u8`  |      |       |       |       |  X   |       |       |       |   X    |
| `u16` |      |       |       |       |  X   |   X   |       |       |   X    |
| `u32` |      |       |       |       |  X   |   X   |   X   |       |   X    |
| `u64` |      |       |       |       |  X   |   X   |   X   |   X   |   X    |

### Others

-   `f32` as `f64`
-   `T` as `&T`
-   `&T` as `*T`
-   `T` as `*T`
-   `T` as `(T)`
-   `(T)` as `T`

## Operators

Most operators that one would expect from a language are provided

### Postfix

-   `;` Evaluates the preceding expression and discards its value

### Unary

-   `&` - Reference
-   `*` - Dereference

### Literal

These operators can only be used on literals and will be evaluated at compile time

#### Unary

-   `~` One's complement (not)

#### Binary

-   `+` - Addition
-   `-` - Substraction
-   `*` - Multiplication
-   `/` - Division
-   `%` - Remainder
-   `|` - Or
-   `&` - And
-   `^` - Xor

## Syntax reference

```ebnf
/* Signs */
Sign ::= [+-]

/* Digits */
Digit    ::= [0-9]
HexDigit ::= [0-9a-fA-F]
OctDigit ::= [0-7]

/* Identifiers and types */
Ident      ::= '_'* Alpha AlphaNum*
Type       ::= TupleT |
               SliceT |
               ArrayT |
               Ref |
               Ptr |
               Fn |
               ( Ident WhiteSpace* Specifiers? )
Specifiers ::= '<' WhiteSpace*
               ( LifeTime CommaSep )*
               ( Lifetime? |
                 ( Type CommaSep )*
                 Type? )
               WhiteSpace '>'
Lifetime   ::= #27 Ident

/* Characters */
Alpha    ::= [a-zA-Z]
AlphaNum ::= [0-9a-zA-Z_]

/* Separators */
NewLine    ::= ( '#0D'? '#0A' )
Spacing    ::= [ #09]
WhiteSpace ::= NewLine | Spacing
CommaSep   ::= WhiteSpace* ',' WhiteSpace*
```
