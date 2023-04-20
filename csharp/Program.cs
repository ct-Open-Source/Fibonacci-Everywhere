using System.Numerics; // BigInteger et al.

// return the Fibonacci sequence
static IEnumerable<BigInteger> fib() {
  (BigInteger prev, BigInteger sum) = (0, 1);
  while(true) {
    yield return sum;
    (prev, sum) = (sum, prev + sum);
  }
}
    
int limit = 100;
if(args.Length > 0) {
  if(int.TryParse(args[0], out int num)) {
    limit = num;
  }
}
foreach(var f in fib().Take(limit)) {
  Console.WriteLine($"{f:n0}");
}

