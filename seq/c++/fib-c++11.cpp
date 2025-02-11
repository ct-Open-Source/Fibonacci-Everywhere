#include <iostream>
#include <vector>

// compile with:
// g++ -std=c++11 -O2 -o fib-11 fib-c++11.cpp

template <typename INT>
std::vector<INT> fibonacci(unsigned int n)
{
    std::vector<INT> result;
    result.reserve(n);
    INT prev = 0;
    INT sum = 1;
    for (unsigned int i = 0; i < n; ++i)
    {
        result.push_back(sum);
        INT prev2 = prev;
        prev = sum;
        sum = prev2 + sum;
    }
    return result;
}

int main(int argc, char *argv[])
{
    unsigned int n = 30;
    if (argc == 2)
    {
        n = static_cast<unsigned int>(std::atoi(argv[1]));
    }
    auto seq = fibonacci<uint64_t>(n);
    for (auto const &element : seq)
    {
        std::cout << element << ", ";
    }
    std::cout << std::endl;
    return 0;
}
