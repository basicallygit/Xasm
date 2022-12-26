# Xasm
A dumbed-down version of assembly with REPL support

# Issues
Due to jmp being its own function that waits for execution to finish, recursive loops will eventually result in a stack overflow.<br>
To get around this, use the LOOP instruction for looping.
