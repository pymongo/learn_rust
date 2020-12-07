#!/usr/bin/python3

import ctypes


class MapPtr(ctypes.Structure):
    pass


lib = ctypes.cdll.LoadLibrary("../target/librust.so")
lib.map_new.restype = ctypes.POINTER(MapPtr)
lib.map_insert.argtypes = (ctypes.POINTER(MapPtr), ctypes.c_int, ctypes.c_int)
lib.map_get.argtypes = (ctypes.POINTER(MapPtr), ctypes.c_int)
lib.map_get.restype = ctypes.c_int
lib.map_free.argtypes = (ctypes.POINTER(MapPtr),)


class Map:
    def __init__(self):
        self.ptr: ctypes.POINTER(MapPtr) = lib.map_new()

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        lib.map_free(self.ptr)

    def insert(self, k: int, v: int):
        lib.map_insert(self.ptr, k, v)

    def get(self, k: int) -> int:
        return lib.map_get(self.ptr, k)


if __name__ == '__main__':
    with Map() as my_map:
        my_map.insert(1, 1)
        print(f"map.get(1) = {my_map.get(1)}")
        print(f"map.get(2) = {my_map.get(2)}")
