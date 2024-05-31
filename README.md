# JAL (Just Another Language)

JAL is a hobby programming language designed for my own amusement. It aims to provide a simple and intuitive syntax while incorporating features from various programming paradigms.

# JAL Syntax

## Comments
```c
// This is a single-line comment

/*
   This is a
   multi-line comment
*/
```

## Variables and Constants
```c
// Variable declarations
int count = 10;
float price = 9.99;
string name = "John";
bool isAvailable = true;

// Constant declaration
const int MAX_VALUE = 100;
```

## Operators
```c
// Arithmetic operators
int sum = 5 + 3;
int difference = 10 - 2;
int product = 4 * 6;
float quotient = 15 / 2;
int remainder = 17 % 3;

// Comparison operators
bool isEqual = x == y;
bool isNotEqual = x != y;
bool isGreater = x > y;
bool isLess = x < y;
bool isGreaterOrEqual = x >= y;
bool isLessOrEqual = x <= y;

// Logical operators
bool result = x < 5 && y > 10;
bool outcome = a == 1 || b == 2;
bool negation = !(p && q);
```

## Control Flow
```c
// If-else statement
if (score >= 60) {
    print("Passed");
} else {
    print("Failed");
}

// Switch statement
switch (grade) {
    case 'A':
        print("Excellent");
        break;
    case 'B':
        print("Good");
        break;
    default:
        print("Average");
}
```

## Loops
```c
// While loop
while (count > 0) {
    print(count);
    count--;
}

// Do-while loop
do {
    print(count);
    count++;
} while (count < 5);

// For loop
for (int i = 0; i < 10; i++) {
    print(i);
}

// For-each loop
for (string fruit of fruits) {
    print(fruit);
}
```

## Functions
```c
// Function declaration
int add = (int a, int b) => {
    return a + b;
}

// Function call
int result = add(5, 3);

// Arrow function
int square = x => x * x;
```

## Arrays
```c
// Array declaration and initialization
int numbers[] = [1, 2, 3, 4, 5];
string fruits[] = ["apple", "banana", "orange"];
```

## Objects
```c
// Object declaration
object Person = {
    string name = "Alice";
    int age = 25;
    
    void greet = () => {
        print("Hello, " + name);
    }
}

// Accessing object properties and methods
Person.age = 26;
Person.greet();
```

## Classes
```c
// Class declaration
class Shape {
    public:
        int width;
        int height;

        constructor(int w, int h) {
            width = w;
            height = h;
        }

        int getArea() {
            return width * height; 
        }
}

// Class extension
class Rectangle extends Shape {
    public:
        constructor(int w, int h) : super(w, h) { } 
}

// Creating an instance of the class
Rectangle rect = new Rectangle(5, 3);
int area = rect.getArea();
```

## Interfaces
```c
// Interface declaration
interface Drawable {
    void draw();
}

// Implementing an interface
class Circle implements Drawable {
    public:
        void draw() {
            print("Drawing a circle.");
        }
}
```

## Imports and Exports
```c
// math.jal
export int PI = 3;

// main.jal
import { PI } from "math";

void main() {
    print(PI);
}
```

## ToDo:
**Phase 1: Lexical Analysis (Scanning) - DONE**

- [x] **Tokenizer (Lexer):** Create a tokenizer using Rust's pattern matching and regular expressions (`regex` crate).
- [x] **Token Data Structure:** Define an enum `TokenType` and a struct `Token`.
- [x] **Error Handling:** Implement basic error handling in the tokenizer.

**Phase 2: Syntactic Analysis (Parsing) - IN PROGRESS**

- [x] **Abstract Syntax Tree (AST):** Define Rust structs or enums to represent AST nodes.
- [x] **Parser:** Implement a parser (Recursive Descent is recommended) to build the AST.
- [x] **Error Handling:** Enhance error handling in the parser.
- [x] **Test the implemented parsing rules:** Create test cases for keywords and invalid syntax.
- [ ] **Add parsing for remaining statements and expressions:**
    - [ ] Function calls
    - [ ] Array literals
    - [ ] Index access
    - [ ] Member access
    - [ ] Assignment expressions
    - [ ] ... 

**Phase 3: Semantic Analysis**

- [ ] **Symbol Table:** Implement a symbol table for variables, functions, and their types.
- [ ] **Type Checking:** Traverse the AST and perform type checking.
- [ ] **Other Semantic Checks:** Check for undeclared variables, duplicate declarations, etc.

**Phase 4: Intermediate Representation (IR) Generation**

- [ ] **Choose an IR:** Select an IR (e.g., LLVM IR).
- [ ] **IR Generator:** Traverse the AST and generate IR code.

**Phase 5: LLVM Code Generation and Optimization**

- [ ] **LLVM Integration:** Use the LLVM API to interface with LLVM.
- [ ] **IR Optimization:** Use LLVM's optimization passes.
- [ ] **Machine Code Generation:** Generate machine code using LLVM.

**Phase 6: Runtime Library**

- [ ] **Basic Functions:** Implement essential runtime library functions in Rust.
- [ ] **Linkage:** Link the compiled JAL code with the runtime library.

**Additional Considerations**

- [ ] **Error Reporting:** Implement a robust error reporting system.
- [ ] **Testing:** Write comprehensive unit and integration tests.
- [ ] **Standard Library:** Design and implement a standard library for JAL.
- [ ] **Optimizations:** Explore and implement additional optimizations.
- [ ] **Garbage Collection:** Choose and implement a garbage collection strategy.


## License

JAL is released under the [MIT License](LICENSE).