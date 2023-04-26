// Author: Oliver Lau <ola@ct.de>

import java.math.BigInteger;

public class Fibonacci {

	static void fib(int n) {
		BigInteger a = new BigInteger("0");
		BigInteger b = new BigInteger("1");
		for (int i = 0; i < n; ++i) {
			System.out.printf("%s ", a.toString());
			BigInteger c = a.add(b);
			a = b;
			b = c;
		}
	}

	public static void main(String[] args) {
		fib(100);
	}
}
