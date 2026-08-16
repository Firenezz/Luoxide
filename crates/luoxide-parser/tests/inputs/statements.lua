-- assignment
x = 1
a, b = 1, 2
a.b[c] = d

-- local declarations
local plain
local one = 1
local x, y = 1, 2
local const <const> = 42
local closing <close> = resource()

-- if / elseif / else
if a then
    f()
elseif b then
    g()
else
    h()
end

-- loops
while cond do
    body()
end

repeat
    body()
until cond

for i = 1, 10 do
    f(i)
end

for i = 10, 1, -1 do
    f(i)
end

for k, v in pairs(t) do
    f(k, v)
end

-- do block
do
    isolated()
end

-- functions
function top_level(a, b) return a + b end
function m.nested.fn() end
function m.class:method(x) return self, x end
local function local_fn(...) return ... end

-- control flow
::top::
goto top

while true do
    break
end

-- return variants (each must be last in its block)
function returns_nothing() return end
function returns_values() return 1, 2 end
