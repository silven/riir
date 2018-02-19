import Curryrs.Types

foreign import ccall "double" double :: I64 -> I64
foreign import ccall "square" square :: I64 -> I64
foreign import ccall "cube" cube :: I64 -> I64

main = putStrLn $ show $ cube $ square $ double 2