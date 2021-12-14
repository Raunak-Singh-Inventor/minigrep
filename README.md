# `minigrep` is a clone of the `grep` cli in rust

Minigrep will **find** a query string in a file.

To **test** it out, clone the project and run `cargo run body poem.txt`.

Your **output** will be: 
```
I'm nobody! Who are you?
Are you nobody, too?
How dreary to be somebody!
```
To do a **case insensitive** search, add `CASE_INSENSITIVE=1` at the front of your `cargo run` command.
