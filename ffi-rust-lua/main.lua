local ffi = require("ffi")

ffi.cdef[[
const char* say_hello();
]]

local lib = ffi.load("lib-rs/target/release/libmylib.so")

local pointer = lib.say_hello()
local result = ffi.string(pointer)

print(result)
