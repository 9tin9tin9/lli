# Low Level Interpreted -- An attempt to create low level interpreted language

## Ideas:

- Memory slots: 1 slot store a `double`. Functions determine how to interprete double
- [0] is reserved for writing output, [1] is reserved for writing system error code
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
- Syntax of primitive Type:
    - Num: `-?[1-9][0-9]*(?:.[0-9]+)?`
    - Idx: Positive integer wrapped in square brackets, no space
    - Var: `$name`
    - Sym: `[^\s0-9$#"\[\]:,]+`
    - Ltl: String literal wrapped in double quotes
- Args:
    - Value: {Num, Idx, Var}
    - Ptr: {Idx, Var, Ltl}
    - Writable Ptr: {Idx, Var}
        - Ltl returns negative ptr, which is unwritable
    - Symbol: {Sym}
- Statement: `Func arg1 arg2 arg3...`
- 1 line per statement
 
## Predefined Functions:

```Python
# memory management
mov: des(WPtr), src(Value)  # assignment, read value
cpy: des(WPtr), src(Ptr), size(Value  # memcpy. When src = Ltl, a new ltl is created and its idx is used as src idx
var: name(Sym), idx(Ptr)  # Creates or update $name with index = idx
incr: var(Var), num(Value)  # Used to iterate->read/write pmem, potentially can be used to do stack operations
decr: var(Var), num(Value)  # Used to iterate->read nmem
allc: size(Value)  # Push slots to pmem

# maths, [0] is set as result
# args can be index, var or Num
add: left(Value), right(Value)  # +
sub: left(Value), right(Value)  # -
mul: left(Value), right(Value)  # *
div: left(Value), right(Value)  # /
mod: left(Value), right(Value)  # %

# cmp, [0] is set to either 0 or 1
eq: left(Value), right(Value)  # ==
ne: left(Value), right(Value)  # !=
gt: left(Value), right(Value)  # >
lt: left(Value), right(Value)  # <

# logic, [0] is set to either 0 or 1
and: left(Value), right(Value)  # &&
or: left(Value), right(Value)  # ||
not: bool(Value)  # !

# control flow
skp: idx(Value) # tests idx and skips a line if true
jmp: lbl(Sym)  # unconditional jump
lbl: lbl(Sym)  # set label. Label of same name stack up
ret  # returns to the last jump label linenum+1

# sys
exit: exit_code(Value)
fork: ???
# for read and write, [0] set to bytes read or wrote
read: fd(Value), ptr(WPtr), size(Value)
write: fd(Value), ptr(Ptr), size(Value)
open: name(Ptr | Sym), access mode(Value)  # [0] sets to fd
close: fd(Value)

# extra
# added for either debug or simplify instructions
print_num: fd(Value, val(Value)

# extern
src script_name:Ltl  # source another file. creates a sparate memory map
```

## TODO
- [ ] Implement all the functions listed in Predefined Function Section

- [ ] Read idx from Var num
    - Idx of `$A` stores 10
    - `[$A]` returns 10

- [ ] Write more tests

- [ ] Added comments

- [ ] Create all the string literals during preprocessing

## Implement note

As the interpreter performs read write directly on file descriptor, the mutex added by rust on stdout and stdin are bypassed. Mutex or related things should be added manually if multithreading is to be implemented in the future.
