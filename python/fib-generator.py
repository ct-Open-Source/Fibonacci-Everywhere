import sys

def fibonacci():
    a, b = 0, 1
    while True:
        yield a
        a, b = b, a + b

def main():
    n = int(sys.argv[1]) if len(sys.argv) >= 2 else 100
    fib = fibonacci()
    for _ in range(n):
        print(next(fib), end=' ')

if __name__ == "__main__":
    main()

