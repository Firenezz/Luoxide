

local simple = {}

local function add(a, longnameforfunandtests)
    return a + longnameforfunandtests
end

simple.sub = function(a, b)
    return a - b
end

function simple:clone(a, b)
    return a:clone(b)
end

simple.add = add

return simple