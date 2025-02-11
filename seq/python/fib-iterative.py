from argparse import ArgumentParser
parser = ArgumentParser(description='Calculate a Fibonacci sequence.')
parser.add_argument('length', type=int, help='How long should the sequence be?')
args = parser.parse_args()

fib = [1, 1]
for i in range(args.length-1):
    fib.append(fib[i] + fib[i + 1])
print(fib[1:])
