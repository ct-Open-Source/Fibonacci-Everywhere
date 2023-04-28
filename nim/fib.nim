# Install BigInt library with
#   nimble install https://github.com/nim-lang/bigints
#
# Compile and run with
#   nim compile -d:release --run fib.nim
#
# Author: Oliver Lau <ola@ct.de>


import bigints

proc fib(n: int) =
  var a = 0.initBigInt
  var b = 1.initBigInt
  for i in 0..n:
    stdout.write a, " "
    let c = a + b
    a = b
    b = c

fib(100)
