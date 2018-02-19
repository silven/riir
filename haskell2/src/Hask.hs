module Hask where

import Curryrs.Types

foreign export ccall fourth :: I64 -> I64
foreign export ccall fifth :: I64 -> I64
foreign export ccall sixth :: I64 -> I64


fourth :: I64 -> I64
fourth x = x * x * x * x

fifth :: I64 -> I64
fifth x = x * x * x * x * x

sixth :: I64 -> I64
sixth x = x * x * x * x * x * x
