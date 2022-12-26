# Xasm
A dumbed-down version of assembly with REPL support

# Issues
Due to jmp being its own function that waits for execution to finish, recursive loops will eventually result in a stack overflow.<br>
To get around this, use the LOOP instruction for looping.

## Usage

## Registers
|Register|Usage|
|---|---|
|R<kbd>0-12</kbd>|General purpose registers|
|P<kbd>0-12</kbd>|Paramater registers, for passing arguments to functions/syscalls|
|RET<kbd>0-12</kbd>|Return registers, for returning values from functions/syscalls|
|L<kbd>0</kbd>|Loop register, defines how many times to jmp to a label|

## Instructions
|Instruction|Usage|
|---|---|
|PUSH <kbd>register</kbd>|Pushes <kbd>register</kbd> onto the stack|
|POP <kbd>register</kbd>|Pops the top value off the stack and into <kbd>register</kbd>|
|MOV <kbd>register1</kbd> <kbd>value/register2</kbd>|Copies the value of <kbd>value/register2</kbd> into <kbd>register1</kbd>|
|INC <kbd>register</kbd>|Increments the value in <kbd>register</kbd>|
|DEC <kbd>register</kbd>|Decrements the value in <kbd>register</kbd>|
|ADD <kbd>register1</kbd> <kbd>value/register2</kbd>|Adds <kbd>value/register2</kbd> to <kbd>register</kbd>|
|SUB <kbd>register1</kbd> <kbd>value/register2</kbd>|Subtracts <kbd>value/register2</kbd> from <kbd>register</kbd>|
|MUL <kbd>register1</kbd> <kbd>value/register2</kbd>|Multiplies <kbd>register1</kbd> by <kbd>value/register2</kbd>|
|DIV <kbd>register1</kbd> <kbd>value/register2</kbd>|Divides <kbd>register1</kbd> by <kbd>value/register2</kbd>|
|CMP <kbd>register1</kbd> <kbd>value/register2</kbd>|Compares <kbd>value/register2</kbd> to <kbd>register</kbd> and sets <kbd>equal_flag, lesser_flag & greater_flag</kbd> accordingly|
|JMP <kbd>label</kbd>|Calls a builtin or user defined function|
|JE <kbd>label</kbd>|Calls label only if <kbd>equal_flag</kbd> is true|
|JNE <kbd>label</kbd>|Calls label only if <kbd>equal_flag</kbd> is false|
|JZ <kbd>label</kbd>|Calls label only if <kbd>zero_flag</kbd> is true|
|JNZ <kbd>label</kbd>|Calls label only if <kbd>zero_flag</kbd> is false|
|JG, JGE, JL, JLE <kbd>label</kbd>|Jump greater, Jump greater than or equal, Jump less than, Jump less than or equal|
|LOOP <kbd>label</kbd>|Calls <kbd>label</kbd> <kbd>L0</kbd> times|

## Builtin functions
|Function|Usage|
|---|---|
|print|prints <kbd>P0</kbd>|
|printline|prints <kbd>P0</kbd> with a newline|
|input|Fetches user input and places it in <kbd>RET0</kbd>|
|debug|Prints out the entire program layout - functions, register states etc|
|exit|Exits the program with exit code <kbd>P0</kbd>
