# Xasm
A dumbed-down version of assembly with REPL support

# Issues
Due to jmp being its own function that waits for execution to finish, recursive loops will eventually result in a stack overflow.<br>
To get around this, use the LOOP instruction for looping.

## Usage

## Registers
|Register|Usage|
|---|---|
|r<kbd>0-12</kbd>|General purpose registers|
|p<kbd>0-12</kbd>|Paramater registers, for passing arguments to functions/syscalls|
|ret<kbd>0-12</kbd>|Return registers, for returning values from functions/syscalls|
|L<kbd>0</kbd>|Loop register, defines how many times to jmp to a label|

## Instructions
|Instruction|Usage|
|---|---|
|push <kbd>register</kbd>|Pushes <kbd>register</kbd> onto the stack|
|pop <kbd>register</kbd>|Pops the top value off the stack and into <kbd>register</kbd>|
|mv <kbd>register1</kbd> <kbd>value/register2</kbd>|Copies the value of <kbd>value/register2</kbd> into <kbd>register1</kbd>|
|inc <kbd>register</kbd>|Increments the value in <kbd>register</kbd>|
|dec <kbd>register</kbd>|Decrements the value in <kbd>register</kbd>|
|add <kbd>register1</kbd> <kbd>value/register2</kbd>|Adds <kbd>value/register2</kbd> to <kbd>register</kbd>|
|sub <kbd>register1</kbd> <kbd>value/register2</kbd>|Subtracts <kbd>value/register2</kbd> from <kbd>register</kbd>|
|mul <kbd>register1</kbd> <kbd>value/register2</kbd>|Multiplies <kbd>register1</kbd> by <kbd>value/register2</kbd>|
|div <kbd>register1</kbd> <kbd>value/register2</kbd>|Divides <kbd>register1</kbd> by <kbd>value/register2</kbd>|
|cmp <kbd>register1</kbd> <kbd>value/register2</kbd>|Compares <kbd>value/register2</kbd> to <kbd>register</kbd> and sets <kbd>equal_flag, lesser_flag & greater_flag</kbd> accordingly|
|jmp <kbd>label</kbd>|Calls a builtin or user defined function|
|je <kbd>label</kbd>|Calls label only if <kbd>equal_flag</kbd> is true|
|jne <kbd>label</kbd>|Calls label only if <kbd>equal_flag</kbd> is false|
|jz <kbd>label</kbd>|Calls label only if <kbd>zero_flag</kbd> is true|
|jnz <kbd>label</kbd>|Calls label only if <kbd>zero_flag</kbd> is false|
|jg, jge, jl, jle <kbd>label</kbd>|Jump greater, Jump greater than or equal, Jump less than, Jump less than or equal|
|loop <kbd>label</kbd>|Calls <kbd>label</kbd> <kbd>L0</kbd> times|
