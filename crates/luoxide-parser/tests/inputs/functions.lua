local function f()
    return 1
end

local function g(a, b, c)
    return a + b() + c .. "d"
end

function obj:method(f, ...)
    goto label
    local args = table.unpack(...)
    local temp = function()
        return 2
    end
    ::label::
    local temp = function()
        return 1
    end
    return f(temp, ...)
end