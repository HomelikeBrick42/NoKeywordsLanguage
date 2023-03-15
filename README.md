# No keywords language

## Types
- `void`
- `bool`
- `int`
- function types (see below)
- struct types (see below)

## Functions:

The function returns whatever value its block returns

Functions are first class, so they are values and usally assigned to constants

### Function Types
`(a: type, b: type) -> type`

### Function Values
`(a: type, b: type) -> type { ... }`

`(a: type, b: type) { ... }`

`$name: type` is a compile time parameter that you can use as generics

a generic identity function:

`($T: type) -> ((value: T) -> T) { (value: T) -> T { value } }`

or if you want to remove the unnessaseary types and let the compiler infer them

`($T: type) { (value: T) { value } }`

### Calling a function:
`func(a, b)`

### Calling the "generic" identity function from before:
`foo(int)(5)`

## Structs

Structs are for grouping values

### Struct Types:
`(a: type, b: type)`

### Struct Values:
`(a, b)`

Creating a struct that has a single value: `(a,)`

## Variables:
`name: type = value`

`name: type`

`name := value`

## Constants:

`name: type : value`

`name :: value`
