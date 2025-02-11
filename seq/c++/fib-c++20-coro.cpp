#include <concepts>
#include <coroutine>
#include <cstdint>
#include <exception>
#include <iostream>
#include <type_traits>

// compile with:
// g++ -std=c++20 -O2 -o fib-20 fib-c++20-coro.cpp

template <typename T>
struct generator
{
    struct promise_type;
    using handle_type = std::coroutine_handle<promise_type>;

    struct promise_type
    {
        T value_;
        std::exception_ptr exception_;

        generator get_return_object()
        {
            return generator(handle_type::from_promise(*this));
        }
        std::suspend_always initial_suspend() { return {}; }
        std::suspend_always final_suspend() noexcept { return {}; }
        void unhandled_exception() { exception_ = std::current_exception(); }

        template <std::convertible_to<T> From>
        std::suspend_always yield_value(From &&from)
        {
            value_ = std::forward<From>(from);
            return {};
        }
        void return_void() {}
    };

    handle_type h_;

    generator(handle_type h)
        : h_(h)
    {
    }
    ~generator() { h_.destroy(); }
    explicit operator bool()
    {
        fill();
        return !h_.done();
    }
    T operator()()
    {
        fill();
        full_ = false;
        return std::move(h_.promise().value_);
    }

private:
    bool full_ = false;

    void fill()
    {
        if (!full_)
        {
            h_();
            if (h_.promise().exception_)
            {
                std::rethrow_exception(h_.promise().exception_);
            }
            full_ = true;
        }
    }
};

template<typename T>
concept Integral = std::integral<T>;

template <Integral INT>
generator<INT> fibonacci(unsigned int n)
{
    INT a = 0;
    INT b = 1;
    for (unsigned int i = 0; i < n; ++i)
    {
        co_yield a;
        INT sum = a + b;
        a = b;
        b = sum;
    }
}

int main(int argc, char *argv[])
{
    unsigned int n = 30;
    if (argc == 2)
    {
        n = static_cast<unsigned int>(std::atoi(argv[1]));
    }

    auto seq = fibonacci<uint64_t>(n);
    while (seq)
    {
        std::cout << seq() << ' ';
    }

    std::cout << std::endl;
    return 0;
}
