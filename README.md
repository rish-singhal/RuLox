# Interpreter for Lox language

## Instructions

1. Compile the program
```bash
$ rustc src/main.rs
```

2. Execute as REPL
```bash
$ ./main
>
```

or give a lox script as input
```bash
$ ./main [FILE_NAME]
```

## Syntax Grammar

### Version 1

```text
expression -> literal
            | grouping
            | unary
            | binary;

literal -> NUMBER
         | STRING
         | "true"
         | "false"
         | "nil";

grouping -> "(" expression ")";

binary -> expression operator expression;

operator -> "*"
          | "/"
          | "-"
          | "+"
          | "=="
          | "!="
          | "<"
          | ">"
          | "<="
          | ">=";
```

## Further Questions

1. Learn more about error codes while exiting the program.
2. [Difference between expression and statement?](https://dev.to/promhize/javascript-in-depth-all-you-need-to-know-about-expressions-statements-and-expression-statements-5k2#:~:text=Expressions%20are%20Javascript%20code%20snippets%20that%20result%20in%20a%20single%20value.&text=All%20of%20the%20above%20are,is%20logged%20to%20the%20console.)

## Sources

1. [Crafting Interpretors: Lox](https://craftinginterpreters.com/)
2. [Rolox: SarcasticNastik](https://github.com/SarcasticNastik/rolox)

## Author

- [Rishabh Singhal](https://rish-singhal.github.io)

