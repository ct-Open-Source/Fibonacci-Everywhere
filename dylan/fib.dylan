define method fib ()
    let a :: <integer> = 0;
    let b :: <integer> = 1;
    method ()
        let r = a + b;
        a := b;
        b := r;
        a
    end
end;

let f = fib();
for (i from 0 to 94)
    format-out("%= ", f())
end
