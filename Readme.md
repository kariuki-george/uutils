# Rewriting existing tools to learn Rust

## Echo

echo's the input back to stdout

### To run

`cargo run --bin echo  [sometext]`

or

`./echo [sometext]` for the binary

## Cat

Print a file contents

### To run

`cargo run --bin cat  filepath`

or

`./cat filepath` for the binary

## Ls

Lists all the contents of a directory

### To run

`cargo run --bin ls  path`

or

`./ls path` for the binary

## Find

Replicates unix find command

### To run

`cargo run --bin find [starting_path]  --[pattern] [value]`

or

`./find [starting_path] --[pattern] [value]` for the binary

### Patterns

- path -> defaults to current directory
- name

* all patterns are optional

## Grep

Replicates unix find command

### To run

`cargo run --bin grep [path(s)]  --[pattern] [value]`

or

`./grep [path(s)] --[pattern] [value]` for the binary

### Patterns

- input
