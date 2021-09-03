# Low Level Interpreted -- An attempt to create low level interpreted language

## Ideas:

- Memory slots: 1 slot store a `double`. Functions determine how to interprete double
- Primitive type: Num, Idx, Var, Lbl, Ltl
- No concept of Stack. Plain memory: Positive Memory and Negative Memory
    - Positive Memory(pmem): idx>=0, stores modifiable data
    - Negative Memory(nmem): idx<0, store and referenced by string literals, read only, set by interpreter
- Allow define functions in future? But still won't have stack

...Maybe I can write a compiler for this using LLVM?
future reference for writing compiler: https://llvm.org/docs/tutorial/MyFirstLanguageFrontend/index.html

## Syntax:

- Comment: everything after hashtag
- Character: single character wrapped in single quote, converts to Num type which value is its ASCII value
- Reserved symbols: `:,"[]$#`
- Arg Type:
    - Num: `-?[1-9][0-9]*(?:.[0-9]+)?`
    - Idx: Positive integer wrapped in square brackets, no space
    - Var: `$name`
    - Sym: `[^\s0-9$#"\[\]:,]+`
    - Ltl: String literal wrapped in double quotes
- Statement: `Func arg1 arg2 arg3...`
- 1 line per statement
 
## Predefined Functions:

```Python
# memory management
mov des:{Idx, Var}, src:{Num, Idx, Var}  # assignment, read value
cpy des:{Idx, Var}, src:{Idx, Var, Ltl}, size:{Num, Idx, Var}  # memcpy. When src = Ltl, a new ltl is created and its idx is used as src idx
var name:{Sym}, idx:{Idx, Var, Ltl}  # Creates or update $name with index = idx
incr var:{Var}, num:{Num, Idx, Var} # Used to iterate->read/write pmem, potentially can be used to do stack operations
decr var:{Var}, num:{Num, Idx, Var}  # Used to iterate->read nmem
allc size:{Num}  # Push slots to pmem

# maths, [0] is set as result
# args can be index, var or Num
add left:{Num, Idx, Var}, right:{Num, Idx, Var}  # +
sub left:{Num, Idx, Var}, right:{Num, Idx, Var}  # -
mul left:{Num, Idx, Var}, right:{Num, Idx, Var}  # *
div left:{Num, Idx, Var}, right:{Num, Idx, Var}  # /

# cmp, [0] is set to either 0 or 1
eq left:{Num, Idx, Var}, right:{Num, Idx, Var}  # ==
ne left:{Num, Idx, Var}, right:{Num, Idx, Var}  # !=
gt left:{Num, Idx, Var}, right:{Num, Idx, Var}  # >
lt left:{Num, Idx, Var}, right:{Num, Idx, Var}  # <

# logic, [0] is set to either 0 or 1
and left:{Num, Idx, Var}, right:{Num, Idx, Var}  # &&
or left:{Num, Idx, Var}, right:{Num, Idx, Var}  # ||
not bool:{Num, Idx, Var}  # !

# control flow
skp idx:{Num, Idx, Var} # tests idx and skips a line if true
jmp lbl:{Sym}  # unconditional jump
lbl lbl:{Sym}  # set label. Label of same name stack up

# IO
in des:{Idx, Var}, size:{Num, Idx, Var}
out src:{Idx, Var, Ltl}, size:{Num, Idx, Var}  # Read and print size number of ascii chars
outa ascii:{Num, Idx, Var}  # Read a signle ascii value and prints it
outv val:{Num, Idx, Var}  # Read and print value (float)
read fd:{Num, Idx, Var}, des:{Idx, Var}, size:{Num, Idx, Var}
write fd:{Num, Idx, Var}, src:{Idx, Var, Ltl}, size:{Num, Idx, Var}

# extern
src script_name:{Ltl}  # source another file. creates a sparate memory map
ext lib:{Ltl}  # calls dlopen. [0] stores CPtr to handle or Num<0 when failure. [1] set to start of Null-ended error msg. Msg length < 999
cls handle:{Idx, Var}  # closes handle
cal handle:{Idx, Var} sym:{Ltl}  # call sym from handle. Error handling same as etn
```

## TODO
- Refactor `lex::Tok::from_string()`
- Read idx from Var num
    - Idx of $A stores 10
    - `[$A]` returns 10
