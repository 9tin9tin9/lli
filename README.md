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
    - Idx: 0 or Positive integer wrapped in square brackets, no space
    - Var: `$name`
    - VarIdx: `[$name]`, no space inside brackets
    - Sym: `[^\s0-9$#"\[\]:,]+`
    - Ltl: String literal wrapped in double quotes
- Args:
    - Value: {Num, Idx, Var, VarIdx}
    - Ptr: {Idx, Var, VarIdx, Ltl}
    - Writable Ptr: ptr with positive index
        - Ltl returns negative ptr, which is unwritable
    - Symbol: {Sym}
- Statement: `Func: arg1, arg2, arg3...`
- 1 line per statement
 
## Predefined Functions:

```bash
# memory management
mov: des(WPtr), src(Value)  # assignment, read value
cpy: des(WPtr), src(Ptr), size(Value  # memcpy. When src = Ltl, a new ltl is created and its idx is used as src idx
var: name(Sym), idx(Ptr)  # Creates or update $name with index = idx
loc: ptr(Ptr)  # writes the ptr as value to [0]
incr: var(Var), num(Value)  # Used to iterate->read/write pmem, potentially can be used to do stack operations.
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
jmp: lbl(Sym)  # unconditional jmp
jc: cond(Value), lbl(Sym)  # jump if cond is true
lbl: lbl(Sym)  # set label.
als: als(Sym) lbl(Sym)  # set alias. als = lbl

# sys
exit: exit_code(Value)
fork: ???
# for read and write, [0] set to bytes read or wrote
read: fd(Value), ptr(WPtr), size(Value)
write: fd(Value), ptr(Ptr), size(Value)
open: name(Ptr | Sym), option(Value)  # [0] sets to fd
close: fd(Value)

# extra
# added for either debug or simplify instructions
print_num: fd(Value, val(Value)

# extern
src: script_name(Sym)  # execute another file
```

## TODO
- [x] Implement nested Idx to replace VarIdx

- [x] Add loc op to obtain ptr idx value

- [ ] Write more tests

- [ ] Create all the string literals during preprocessing??

- [ ] Move argc check to preprocess time

## Implement note

As the interpreter performs read write directly on file descriptor, the mutex added by rust on stdout and stdin are bypassed. Mutex or related things should be added manually if multithreading is to be implemented in the future.
