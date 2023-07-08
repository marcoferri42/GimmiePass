# PassGen - Command Line Password Generator

Introducing PassGen, a simple password generator tool made in Rust, born out of the necessity for effortless security.

## Usage

```
pass_gen [password length] [y/n for numbers] [y/n for special chars] [y/n for caps]
```

If no fields are specified, the tool will generate a password using all available character types.

## Examples

Generate a 12-character password with numbers, special characters, and capital letters:

```sh
pass_gen 12 y y y
```

Or create a 30-character password with numbers and capital letters:

```sh
pass_gen 30 y n y
```

## Assistance

To view a help message, enter:

```sh
pass_gen -h
```

## Note

Rest assured, no passwords are stored or recorded, I dont even remember what I had for lunch yesterday. 
