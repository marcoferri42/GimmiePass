# GimmiePass ⚿ - Command Line Password Generator

Introducing GimmiePass, a simple password generator tool made in Rust 🦀, born out of the necessity for effortless security (and to learn the language).

## Features

- Generated password gets copied straight in your clipboard! (rust-clipboard crate)
- Customize the chars used in your password generation!
## Usage

```
$ gimmiepass [password length] [y/n for numbers] [y/n for special chars] [y/n for caps]
```

If no fields are specified, the tool will generate a password using all available character types.

---
## Examples

### 20-character password with numbers, special chars, and capital letters:

```sh
$ gimmiepass 20

]vSEW£vzO)0XOI]|yNSO
```

### or

```sh
$ gimmiepass 20 y y y

£O=]RcPuOZib1abd7fgj
```

### 30-character password with numbers and capital letters:

```sh
$ gimmiepass 30 y n y

9icpuMM9oq2CXPEdI2v9p2dfTa2A5b
```

---

## HELP!

To view a help message, enter:

```sh
$ gimmiepass -h
```

---

## Security

Rest assured, no passwords are stored or recorded, I dont even remember what I had for lunch yesterday. 
