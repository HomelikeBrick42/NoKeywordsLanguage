# No keywords language

## Types
- `void`
- `bool`
- `int`
- Union types (see below)
- Function types (see below)
- Struct types (see below)

## Functions

The function returns whatever value its block returns

Functions are first class, so they are values and usally assigned to constants

### Function types
`(a: type, b: type) -> type`

### Function values
`(a: type, b: type) -> type { ... }`

`(a: type, b: type) { ... }`

`$name: type` is a compile time parameter that you can use as generics

a generic identity function:

```
($T: type) -> ((value: T) -> T) {
    (value: T) -> T {
        value
    }
}
```

or if you want to remove the unnessaseary types and let the compiler infer them

```
($T: type) {
    (value: T) {
        value
    }
}
```

### Calling a function
`func(a, b)`

### Calling the "generic" identity function from before
`foo(int)(5)`

## Structs

Structs are for grouping values

### Struct types
`(a: type, b: type)`

### Struct values
`(a, b)`

Creating a struct that has a single value: `(a,)`

### Getting the member values

You can use `the_struct.name` to get the struct member by-name

"Indexing" a struct gives you a union of all the possible types in the struct

```
foo :: (a: int, b: bool)

index: int = ...
value: int | bool = foo.(index)
value2: bool = foo.1 // this index is a compile-time value, so the type is always bool
```

## Union types

`a | b | c`

to extract a value from a union you can use `the_union_value.(type)`

## Variables
`name: type = value`

`name: type`

`name := value`

## Constants

`name: type : value`

`name :: value`

## Compile time values

Compile time values are expressions that do not involve any local variables

### Examples of compile time values
- `5`
- `10 + 10`
- `some_function(some_constant)`

### Examples of things that are not compile time values
- `some_local_variable`
- `some_function(some_local_variable)`
