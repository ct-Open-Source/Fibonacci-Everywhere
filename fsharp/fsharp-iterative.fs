open System

// run with
//
//   dotnet fsi <fsharp-iterative.fs 

let rec fib (n: int): bigint =
    match n with
    | 0 -> 0I
    | n ->
        let mutable last = 0I
        let mutable next = 1I
        for i in 1 .. (n - 1) do
            printf "%A " next
            let temp = last + next
            last <- next
            next <- temp
        next

fib 100 

