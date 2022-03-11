# Interpreter for Lox language

## TODOS

- [ ] Add tests for the interpreter, parser and lexer
- [ ] Handle case `print ;`

## Instructions

1. Compile the program
```bash
$ rustc src/main.rs -o lox
```

*NOTE:* To compile in debug mode, use the following command:

```bash
$ rustc src/main -o lox --cfg debug_lox
```

[Conditional Compilation](https://doc.rust-lang.org/reference/conditional-compilation.html)

2. Execute as REPL
```bash
$ ./lox
>
```

or give a lox script as input
```bash
$ ./lox [FILE_NAME]
```

## Syntax Grammar

### Version 4 (global variables)

```text
program -> declaration* EOF;

declaration -> varDecl
            | statement;
varDecl -> "var" IDENTIFIER ("=" expression)? ";";
primary -> NUMBER | STRING | "true" | "false" | "nil"
        | "(" expression ")"
        | IDENTIFIER;

expression -> assignment;
assignment -> IDENTIFIER "=" assingment
            | equality;
```

### Version 3 (Statements)

```text
program -> statement* EOF;

statement -> exprStmt
            | printStmt;
exprStmt -> expression ";" ;
printStmt -> "print" expression ";";
```

### Version 2 (Operator Precedence)

```text
expression -> equality ;
equality -> comparison ( ( "!=" | "==" ) comparison )* ;
comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term -> factor ( ( "-" | "+" ) factor )* ;
factor -> unary ( ( "/" | "*" ) unary )* ;
unary -> ( "!" | "-" ) unary
               | primary ;
primary -> NUMBER | STRING | "true" | "false" | "nil"
               | "(" expression ")" ;
```

### Version 1

The bellow grammar would be used to create a syntax tree which is abstract
 (AST).

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
3. [What is dynamic dispatch and static dispatch?](https://lukasatkinson.de/2016/dynamic-vs-static-dispatch)

### Questions specific to Rust

1. [Generic returns in Rust](https://blog.jcoglan.com/2019/04/22/generic-returns-in-rust/)
2. [Variadic functions](https://stackoverflow.com/questions/28951503/how-can-i-create-a-function-with-a-variable-number-of-arguments)

## Resources

1. [Crafting Interpretors: Lox](https://craftinginterpreters.com/)
2. [Rolox: SarcasticNastik](https://github.com/SarcasticNastik/rolox)

### Rust Resources

1. [Unofficial Rust Docs](https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html)
2. [How Rusty is your Rust: Solana Resource](https://safari.study/courses/how-rusty-is-your-rust-lang/)

## Author

- [Rishabh Singhal](https://rish-singhal.github.io)

