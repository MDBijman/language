# Language
A small lexer/parser/compiler project to learn about programming language design and implementation. The design of the language is inspired mainly by Haskell and C++. The goal is to be fast and expressive. 

Current Features
 - Control flow: if-else, while 
 - Scoping, typing, name resolution
 - Sum types and product types
 - Arrays
 - Logical and arithmetic operators
 - Simple io

Planned Features
 - Typeclasses
 - Generics
 - FFI
 
## Implementation

The implementation of the different pipeline stages can be found in Compiler/{src, include}/fe/pipeline. 
 
## Snippets

Simple control flow
```
let a: std.ui64 = 1;
if (true) { a = 2; } else { a = 3; };
std.io.print a;

let b: std.ui64 = 5;
while (b > 2) { b = b - 1; };
std.io.print b;
```

Sum types and match expressions
```
let val: Num: std.ui64 | Bool: std.bool = Num 3;

val match {
    | Bool x -> { std.io.print "bool"; }
    | Num x -> { std.io.print x; }
};
```

Nested scoping
```
let x: std.ui64 = {
    let a: std.ui64 = 1;
    let b: std.ui64 = 2;
    let c: std.ui64 = a + b;
    c
};
std.io.print x
```

Tuple destructuring
```
let a : (std.ui64, std.ui64) = (3, 5);
let (b, c): (std.ui64, std.ui64) = a;
std.io.print b;
```

Arrays
```
let y: [std.ui64; 3] = [1,2,3];
let second: [std.ui64; 3] -> std.ui64 = \arr => (arr!!1);
std.io.print (second y);
```

Fib
```
let fib: std.ui64 -> std.ui64 = \n => if (n <= 2) { 1 } else { fib (n - 1) + fib (n - 2) };
let a: std.ui64 = fib 31;
std.io.println a;
```
