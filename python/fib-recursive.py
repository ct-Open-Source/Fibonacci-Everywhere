import argparse


def fibonacci(n: int):
    return fibonacci(n - 1) + fibonacci(n - 2) if n > 1 else n


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Calculate a Fibonacci sequence.')
    parser.add_argument('length', type=int, help='How long should the sequence be?')
    args = parser.parse_args()
    print([fibonacci(n) for n in range(1, args.length + 1)])
