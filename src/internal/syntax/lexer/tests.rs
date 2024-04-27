use super::*;

#[test]
fn lex_file() {
    let input = r##"
function add(a, b)
    return a + b
end

v = add(0, 1)

local function fact(n)
    if n < 2 then return n else return n * fact(n - 1) end
end

function variadic(...)
    return ...
end
"##;

    let interner = Rc::from(DefaultInterner::default());
    let lexer = Lexer::new(input, interner.clone());
    let tokens = TokenVec(
        Tokens(lexer)
            .map(|(string, token)| DisplayToken(token, string))
            .collect::<Vec<_>>(),
    );

    println!("{}", tokens);
}

#[test]
fn lex_design_file_function() {
    let input = r##"
function add(a, b)
    return a + b
end

v = add(0, 1)

local function fact(n)
    if n < 2 then return n else return n * fact(n - 1) end
end

function variadic(...)
    return ...
end

print("print('Hello World')")

-- loops
-- range is [0, 10)
do
    for i = 0, 10 do
        print(i)
    end

    for i in {0, 10} do
        print(i)
    end

    v = 0
    while v < 10 do
        print(v)
        v = v + 1
    end

    goto conditions

    repeat
        print(v)
        if v == 5 then
            break
        end
        v = v + 1
    until v > 10

    ::conditions::

    -- conditionals
    if v < 10 then
        print("less than 10")
    elif v == 20 then
        print("equal to 20")
    else
        print("v is " .. v)
    end

end

print("exp " .. (a and assert(b) or c))
"##;
    let interner = Rc::from(DefaultInterner::default());
    let lexer = Lexer::new(input, interner.clone());
    let tokens = Tokens(lexer)
        .map(|(string, token)| DisplayToken(token, string))
        .collect::<Vec<_>>();

    assert_debug_snapshot!(tokens)
}

#[test]
fn lex_design_file() {
    let input = r##"
-- values
v = true            -- bool
v = 1               -- number
v = "hello"         -- string
v = { a = "hello" } -- table
v = { [f(1)] = g; "x", "y"; x = 1, f(x), [30] = 23; 45 } -- table

-- integers
3
345
0xff
0xBEBADA

-- floats
3.0
3.1416
314.16e-2
0.31416E1
34e1
0x0.1E
0xA23p-4
0X1.921FB54442D18P+1
NaN

-- operators
v = 1 + 1               -- Integer addition
v = 1 - 1               -- Integer subtraction
v = 1 * 1               -- Integer multiplication
v = 1 / 1               -- Float division
v = 1 // 1              -- Integer division
v = 1 % 1               -- Modulo
v = 1 ^ 1               -- Exponentiation
v = 1 << 1              -- Bitwise shift left
v = 1 >> 1              -- Bitwise shift right
v = 1 & 1               -- Bitwise and
v = 1 | 1               -- Bitwise or
v = 1 ~ 1               -- Bitwise xor
v = -1                  -- Negation
v = ~1                  -- Bitwise not
v = "hello" .. "world"  -- String concatenation
v = #"hello world"      -- String lenght

v = ...                 -- Vararg

-- relational operators
v = 1 == 1 -- equal
v = 1 ~= 1 -- not equal
v = 1 < 1  -- less than
v = 1 <= 1 -- less than or equal
v = 1 > 1  -- greater than
v = 1 >= 1 -- greater than or equal

-- logical operators
v = true and true
v = true or true
v = not true

v = 10 or 20        --> 10
v = 10 or error()   --> 10
v = nil or "a"      --> "a"
v = nil and 10      --> nil
v = false and error() --> false
v = false and nil   --> false
v = false or nil    --> nil
v = 10 and 20       --> 20

-- functions
v.a
v["a"]
v(a)

function add(a, b)
    return a + b
end

v = add(0, 1)

local function fact(n)
    if n < 2 then return n else return n * fact(n - 1) end
end

function variadic(...)
    return ...
end

print("print('Hello World')")

-- loops
-- range is [0, 10)
do
    for i = 0, 10 do
        print(i)
    end

    for i in {0, 10} do
        print(i)
    end

    v = 0
    while v < 10 do
        print(v)
        v = v + 1
    end

    goto conditions

    repeat
        print(v)
        if v == 5 then
            break
        end
        v = v + 1
    until v > 10

    ::conditions::

    -- conditionals
    if v < 10 then
        print("less than 10")
    elif v == 20 then
        print("equal to 20")
    else
        print("v is " .. v)
    end

end

print("exp " .. (a and assert(b) or c))

;
;;
;;;
;;;;
;;;;;
    "##; // TODO: add more test cases

    let interner = Rc::from(DefaultInterner::default());
    let lexer = Lexer::new(input, interner.clone());
    let tokens = TokenVec(
        Tokens(lexer)
            .map(|(string, token)| DisplayToken(token, string))
            .collect::<Vec<_>>(),
    );

    assert_debug_snapshot!(tokens)
}

#[test]
fn lex_string_file() {
    let input = r##"
'alo\n123"'
"alo\n123\""
'\97lo\10\04923"'
[[alo
123"]]
[==[
alo
123"]==]
    "##;

    let interner = Rc::from(DefaultInterner::default());
    let lexer = Lexer::new(input, interner.clone());
    let tokens = Tokens(lexer)
        .map(|(string, token)| DisplayToken(token, string))
        .collect::<Vec<_>>();

    assert_debug_snapshot!(tokens)
}
