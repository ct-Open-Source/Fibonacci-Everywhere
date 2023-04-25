#include <iostream>
#include <coroutine>

// compile with:
// g++ -std=c++20 -O2 -o fib fib-c++20-coro.cpp

template <typename T>
class [[nodiscard]] generator
{
public:
    struct promise_type;
    using coro_handle_type = std::coroutine_handle<promise_type>;

private:
    coro_handle_type coro;

public:
    explicit generator(coro_handle_type h) : coro{h} {}
    generator(const generator &) = delete;
    generator &operator=(const generator &) = delete;
    generator(generator &&oth) noexcept : coro{std::move(oth.coro)}
    {
        oth.coro = nullptr;
    }
    generator &operator=(generator &&other) noexcept
    {
        if (this != &other)
        {
            if (coro)
            {
                coro.destroy();
            }
            coro = std::move(other.coro);
            other.coro = nullptr;
        }
        return *this;
    }
    ~generator()
    {
        if (coro)
        {
            coro.destroy();
            coro = nullptr;
        }
    }

public:
    bool has_next()
    {
        if (!coro || coro.done())
        {
            return false;
        }
        coro.resume();
        return !coro.done();
    }

    T value()
    {
        return coro.promise().current_value;
    }

public:
    // implementation of above opaque declaration promise_type
    struct promise_type
    {
    private:
        T current_value{};
        friend class generator;

    public:
        promise_type() = default;
        ~promise_type() = default;
        promise_type(const promise_type &) = delete;
        promise_type(promise_type &&) = delete;
        promise_type &operator=(const promise_type &) = delete;
        promise_type &operator=(promise_type &&) = delete;

        auto get_return_object()
        {
            return generator{coro_handle_type::from_promise(*this)};
        }

        auto initial_suspend()
        {
            return std::suspend_always{};
        }

        auto final_suspend() noexcept
        {
            return std::suspend_always{};
        }

        auto return_void()
        {
            return std::suspend_never{};
        }

        auto yield_value(T some_value)
        {
            current_value = some_value;
            return std::suspend_always{};
        }

        void unhandled_exception()
        {
            std::terminate();
        }
    };
};

template <typename INT>
generator<INT> fibonacci(unsigned int n)
{
    INT prev = 0;
    INT sum = 1;
    for (unsigned int i = 1; i < n; ++i)
    {
        co_yield sum;
        INT tmp = prev;
        prev = sum;
        sum = tmp + sum;
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
    while (seq.has_next())
    {
        std::cout << seq.value() << ' ';
    }

    std::cout << std::endl;
    return 0;
}
