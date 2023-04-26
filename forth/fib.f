( Print 1st n numbers of Fibonacci sequence )
(                                           )
( Author: Oliver Lau <ola@ct.de>            )
(                                           )
( run with                                  )
(   gforth < fib.f                          )

: fib ( n -- )
cr
1 1 0        \ start values for sequence
3 roll       \ pick loop limit
0 do
  dup .      \ print current number
  + dup rot  \ calculate next number
loop
drop drop drop ;

93 fib  \ 94th Fibonacci number exceeds 64 bit limit
