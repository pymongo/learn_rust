#!/usr/bin/ruby

require 'ffi'

module MyLib
    extend FFI::Library
    ffi_lib "../target/debug/libc_call_rust_lib.dylib"
    attach_function :map_new, [], :pointer
    attach_function :map_insert, [:pointer, :int, :int], :int
    attach_function :map_get, [:pointer, :int], :int
    attach_function :map_free, [:pointer], :void
end

map = MyLib.map_new
MyLib.map_insert(map, 1, 1)
p MyLib.map_get(map, 1)
MyLib.map_free(map)
