𝔜𝔄ℭ𝔈 Template Options
=====================

𝔜𝔄ℭ𝔈 have classes which can be tuned with certain options — main ones are ::𝘆𝗮𝗰𝗲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬 which include such important knobs as
`sanity_checks` (which enables internal checks) and `no_exceptions` (which is vital to offer reporting using exceptions for tests
and using asserts for production).

[TOC]

# C++20 Approach # {#Cxx20_Approach}

But how can we pass them into a given class? In a post-C++20 world we can just
[declare class](https://godbolt.org/#z:OYLghAFBqd5QCxAYwPYBMCmBRdBLAF1QCcAaPECAM1QDsCBlZAQwBtMQBGAFlICsupVs1qhkAUgBMAISnTSAZ0ztkBPHUqZa6AMKpWAVwC2tQVvQAZPLUwA5YwCNMxENwBspAA6oFhdbT1DE0FvXzU6Kxt7IycXd0VlTFV/BgJmYgJA41NOBJVw2lT0gki7R2dXDwU0jKzg3Ori0ujYyoBKRVQDYmQOAHIpAGZrZEMsAGpxQZ1kavx6KexxAAYAQRXV2mYjTAVPZl7JgDFl8QBOABFzgA5xa7dxE/Ors4B2cVWlp8vz9%2BlT74vP6SSavWRrDajZgKBTHU4/M6nVa3QHnJFnR7wl5Iq6oxEfSSY55oj4orEk1bvPFInSg8GrcaeAwOVh4ZAgDbjcZoWjVcYOVD6cYKESEACeAH1kAgkgBrBRTelcnl8gVC2ioCWYAAevU8BQVg3p4leVyNGw2Wx2ewOmDhxLOt2uAFYiQijuJpNw3S8PdIQSbjWsoTD7QikWSHeifRTceT8atCdTSTGE1T4zS6ZymSy2Ry1lytMYw9jPZxUx6zg88ZXuBLU0jqxmU8mfq3sOMQONrARrhKCFmC1yS%2BdvdJnfcJZwPJzh6mxxO3PW8d6zl9497VuWVx9txuPkdp8v91v5%2Bd1w6x3vL%2BfBpJSLOuWfx5Oz2uz6ed5%2BT0c78eb9%2BN7vju0jXgiq7YNOD5Dk%2BIGLhKd7QQyc5wZObjekOJpmkq3J0HyDZlhW5xNg6tb/uGHwkRRyIEW2zZruM2qTkYGCYIqFqmuxEIQqaXIAPR8eMVq7Pshypk6ro1p63pSf6FrBsIoYEaBRFVqpdYETRyarBi2lxlGBKaZG1HpgZqy0l2OassgnYcniTpUUCHwXgifwAvGwIgHZzZaT5uk%2Bfp1FJj5xmlpSmk6KQjLMtZtniXcjm/M5qZuSlnqSF58UukRXo5Rl3lmaFFL%2BWZgVhcFhWaaZ1G0oGWFcesawEJgRieMIzWTNMzAGEQI5nIMnpFf1nolQiA3SGV5zjRVY2Dam43VS8406IskKKbCqaEvcm3nKNLxbYlZxbTJ8aEmpeJnZJp0khdU0NRs3W9Z4xCoL0MIQDt223Wce3nAdn0nQ6Z2HZdO34t9A3TMpYG%2BsR6nkWFQ3RnpmkzYjVURXVMHDjjuNPmCAB0Iq0OKUoysg8qdRc4wEMQBiYFFBMalquqYPq/gbYM1O0/TWFIXjAs44GBNMUuLETFM1NZVdpHSXlmXaUjHy/Qmk0JmjFJK%2BF2k6Ar%2B4vkub4uS8m4w6Ou4fr%2BkgI%2BbgHgeez5m2cEHTvVnGDNgABU4w6kkPUkG0j7jAAtIs4wAG6oHg6D3Wsj2oDTuwEB92M9vzKHXV910/Z9IN3IDCLA2DMuFzd2eQytyYqVJ53xmRRmaSrOKow32mLRSK1gkHQuE8TpPSnKnPc3TDPjEzmo6nqBpUzTI9893c6E6LEri3akt9RJOUF7D/p65V2lNx8atIhrCZa%2B3Ca6wV9sGzbzsO1%2BTum5bf4fk/D/6%2B/a6u5x7tez72o/ZEGIG0EOYdI7R0HMhZ6r1dgKAgL7ZA/sQH3U4txdYvFxgCSEtsEStoN4JTSp8NK/wSGEjWH0DorAQB9GdH0Ugpg%2BjLHoagGhK0ZByGFF0Hoa9JCDE4PQggNDmFtA6LKEAzplhCBodwehRguDLCkYw5hpBWF9HoQoEAUihFMMoaQOAsAkBoFangdgZAKAQGMZ4UxFRgCcGWPeKgpjmrEE0RABwwj6EOGsOkMUNCBGkGMTsegAB5WgrA/G6NIFgIwIhgDsH8fQ/AxAkhqHDrsTxpBEE9X6AEnsyhElCDwA4Ygvi9BYEybTPA8i%2BgCI6DQegTA2AcB4PwEAgwhBxJQHIOQRSHCaMgB0VA7M8I0ODiEkEwc5iSwkBwmQkgVFKHyP4TQ2g6g5CydoZo5QXC5FCH4Og6yQg%2BAObQbZMQKgNESMkOgRRaj6GyIIJZqSUg1BKNYMoFzdmKDeUchobzzmtE4B0BQ3DehcCoTQuhDDMlqNFsHdC4xgDIBsvYgmIIIC4EICQSYfDcjjD0CYsxuLBiDFAew2QMhBGeNEaQGUzAsAuAgGIiRUjqF9FkTCqJaiNFaNIDokRkK%2BiSHocolhNDqW6Npek1xKzuBAA%3D%3D)
with these options:

~~~{.cc}
    template <auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼>
    class 𝒆𝒙𝒆𝒄𝒖𝒕𝒐𝒓;

    auto process(
        𝒆𝒙𝒆𝒄𝒖𝒕𝒐𝒓<𝐱𝟖𝟔_𝐨𝐩𝐭𝐢𝐨𝐧𝐬{
            {.sanity_checks = true, .no_exceptions = true},
            {.x86_mode = 𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔵86_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞16}}>* executor)
        -> void;

    auto test(
        int,
        𝒆𝒙𝒆𝒄𝒖𝒕𝒐𝒓<𝐱𝟖𝟔_𝐨𝐩𝐭𝐢𝐨𝐧𝐬{
            {.sanity_checks = true, .no_exceptions = true},
            {.x86_mode = 𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝐱𝟖𝟔_𝐦𝐨𝐝𝐞::𝔵86_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞16}}>* executor)
        -> void {
      process(executor);
}
~~~

Note that unlike pre-C++17 world we don't need to specify type of options which we are passing — this means that instead of one
huge `𝐨𝐩𝐭𝐢𝐨𝐧𝐬` class which contains all possible options for all possible components and subcomponents we can use hierarchy and
at the definition site could cleanly separate different kinds of options.

But, unfortunatelly, we are not in the world where C++20 is widely used yet. We need C++17 solution.

# C++17 Way # {#Cxx17_Approach}

C++20 approach wouldn't work for us just yet, thus we are doing
[something different](https://godbolt.org/#z:OYLghAFBqd5QCxAYwPYBMCmBRdBLAF1QCcAaPECAM1QDsCBlZAQwBtMQBGAFlICsupVs1qhkAUgBMAISnTSAZ0ztkBPHUqZa6AMKpWAVwC2tEJICspLegAyeWpgByxgEaZiIbpNIAHVAsJ1Wj1DEzNLPwC1OjsHZyM3Dy9FZUxVIIYCZmICEONTCxSVaNpM7IJYp1d3T28FLJy8sML68sr4xNqASkVUA2JkDgByKQBme2RDLABqcVGdZHr8ejnscQAGAEENzdpmI0wFH2ZB2YAxdfEATgARa4AOcXuANnEL67urgHZxTbX327XH7SS4Az7AySzL6yLY7SbMBQKc6XQFXS6bR5g67oq5vFGfdF3LFo36SPEfbG/TH4ymbH7E9E6KEwzbTHwGFysPDIEA7abTNC0erTFyofTTBQiQgATwA%2BsgEGkANYKOYs/mC4Wi8W0VCyzAAD0GPhKqtGLPEXzu5p2Oz2ByOJ0wyIpV0e93M5NRZ3E0m4Xs%2BPukkMtFq28MRLtR6OprpxAdpRJpJM2ZIZVITKfpycZzL57M53N5W35WmMUYJvs4mZ9V1exNr3FlmfR9ZzGfTgM72GmIGm9gI91lBDzJf5Feu/uk5hess4z1IfPHmanM%2BezeJ/qu/2T/s21c3vwPu9%2BZ3nG5P%2B5X1x3rqnx7vN9G3iX/Ov09n1%2B316vh9/l7OZ8L0ff9H2/Q9pAfVEt2wedFzHN8ILXWVn3g1llyQ2dnn9MdLWtdUBToYUWyrGtrjbV1G2A6NfgomiMRIrt223aYDVnIwMEwNVbStbjYVhK1%2BQAeiE6Z7UOY5Tkzd1PQbX1/Tk4NbXDYRIxIyCyLrTSmxIhj002XF9KTONSV02N6OzEzNiZPsCy5ZBe15Yl3To8FflvVFgVBZMIRAJz2z0gLDIC4z6LTALzMrOldJ0Ug2Q5ezHOkp5XKBdzMy8jLfUkPzko9Mi/QKnL/KsyLaWCqzQqi8LSt0yz6KZUM8L47YtgITAjB8YR2tmeZmAMIgJyuUZfTK4bfQq1ERukKrrmmmqptGzNpvqz5pp0VY4VUpFMzJF5duuSbPj21Krj2hTkzJLTiSu2TLspG65panZ7C5BxCKFdqDR8YghsuDS5Ou5MqLM3SjsTXSFqisb0VW2lGsw9cvw8z49yg1Gjx/QDJGojHQOgm933Rycb3PH8Nr/WarnvH9Tr3CnL1kCD1l6m5R3Qt9oQAOklWgZXlRVkBVVnpgIYgDEwOKud1fUjUwE0gh20Y2bFiW8LQ8dmS5tj1w4mY5jZvK7so%2BSity/SYd%2BcGUyp9EodpS3ov0nRzZPD8kcPcDL2J6nMb/bHcZJ/GMa9x8AZPbd52a3ibX4zZ%2BsGn7UEGREIAO/bHqua2TvTi7XSu07boOklM5G%2BYpDp31kORn8fbRrGgNrmuILr0mPcvBmQKpmm/0r6yfyZt3LlGbAACppkNNIBpILppgAWlWaYADdUDwdBnq2BPUFFw4CAgAc4vTwvDqP3Pi%2BPq5jdRK7vPzp7y8kSv3cD33Q4J4Og9bzYA6bz2UZJ8OYFYLtxAp3d%2B3dSK93JgPd8w8x4TwNFPIgxBZ4LxHsvVe6B2b8iTinBQEBJ7IGnig56vE454WEqJcSjopLORSllP4WUQRMMkEMHorAQBDHMEMUgpghjrB4agThG0ZByAlH0AYzoxicB4QQThAiug9CVCAcw6whCcO4DwowXB1hqL4QI0gQihg8IUCANRcj%2BFsNIHAWASA0CdTwOwMgFAID2J8I4mo8JRCcF0ZwUgVBHHtWIKYiALh5E8JcPYbI0pOEyNIPYg49AADytBWAxMsaQLARgRDAHYLEnh%2BBiBpDUEvQ44SrCIKIe1fJ5B6DKBqVyFwxBol6CwOUsWeBtFDBkT0Gg9AmBsA4DwfgIBRhCBySgOQcghB4BcKYyAPRUAKyIpwueSTIRzyWAbCQoiZCcC%2BCY1I6QNAEO0E0UwfjrDtGqB4PxkRAh0HOYIe5JRrkJBqH4pQxQMgNFyPofIggvnFJ%2BW0ewVR3m3MUL8p5nzflvM6JwHoCgJGDC4Owzh3DeHlKMTrOe2EBTCFENMHxXN1hc04NMCAuBCAkFmJIUYfjph6AcU4uloxRizxEbIGQsjwmKNIIqZgWAPAQCUSotRHChiaKxRkoxJizGkAsQo9FQxWEyoMXKxVfKeilOCUETwQA%3D%3D%3D):

~~~{.cc}
    template <auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼>
    class 𝒆𝒙𝒆𝒄𝒖𝒕𝒐𝒓;

    inline constexpr 𝐱𝟖𝟔_𝐨𝐩𝐭𝐢𝐨𝐧𝐬 𝔵86_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞16_𝔬𝔭𝔱𝔦𝔬𝔫𝔰 = {
        {.sanity_checks = true, .no_exceptions = true},
        {.x86_mode = 𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝔵86_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞16}};

    auto process(𝒆𝒙𝒆𝒄𝒖𝒕𝒐𝒓<&𝔵86_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞16_𝔬𝔭𝔱𝔦𝔬𝔫𝔰>* executor) -> void;

    auto test(int, 𝒆𝒙𝒆𝒄𝒖𝒕𝒐𝒓<&𝔵86_𝔞𝔡𝔡𝔯32_𝔡𝔞𝔱𝔞16_𝔬𝔭𝔱𝔦𝔬𝔫𝔰>* executor) -> void {
      process(executor);
    }
~~~

Use of constants is actually good idea even in C++20 world because it makes it's easier to see which options are used in which
cases, but in the C++17 world it's actually the only possibility since we couldn't just pass options themselves as non-type
template parameter — but can pass address of `inline constexpr` variable.

Yet we now have a subtle difference: how could we **use** these options if they are passed differently in C++20 mode and in C++17
mode?

If we would try to check, e.g., `no_exceptions` value using `𝓸𝓹𝓽𝓲𝓸𝓷𝓼->no_exceptions` then it would work in C++17 mode but wouldn't
allow us to support C++20 approach and if we would try to use `𝓸𝓹𝓽𝓲𝓸𝓷𝓼.no_exceptions` then this just wouldn't work in C++17 mode at
all.

# SFINAE-based approach to reading values # {#SFINAE_Access}

Instead of trying to access values of options directly we are using, instead, flexible
[SFINAE](https://en.cppreference.com/w/cpp/language/sfinae)-based approach. E.g. that's how 𝔫𝔬𝔢𝔵𝔠𝔢𝔭𝔱 is defined:

~~~{.cc}
    template <auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼, typename = void>
    inline constexpr auto 𝔫𝔬𝔢𝔵𝔠𝔢𝔭𝔱 = false;

    template <auto 𝓸𝓹𝓽𝓲𝓸𝓷𝓼>
    inline constexpr auto
        𝔫𝔬𝔢𝔵𝔠𝔢𝔭𝔱<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, decltype((𝓸𝓹𝓽𝓲𝓸𝓷𝓼.no_exceptions, void()))> =
            𝓸𝓹𝓽𝓲𝓸𝓷𝓼.no_exceptions;

    template <auto* 𝓸𝓹𝓽𝓲𝓸𝓷𝓼>
    inline constexpr auto
        𝔫𝔬𝔢𝔵𝔠𝔢𝔭𝔱<𝓸𝓹𝓽𝓲𝓸𝓷𝓼, decltype((𝓸𝓹𝓽𝓲𝓸𝓷𝓼->no_exceptions, void()))> =
            𝓸𝓹𝓽𝓲𝓸𝓷𝓼->no_exceptions;
~~~

First line provides default value (which is used if options passed don't explicitly specify if exceptions should be used or not),
first specializations handles C++20 version and last specialization handles C++17 version.

Note: use of `decltype` (instead of `std::enable_if_t`) and `auto*` in 2nd specialization makes that code compatible with
Visual Studio (version 2019 v16.6+ which is the earlies 𝔜𝔄ℭ𝔈-supported version of Visual Studio).

And to reduce code duplication we provide `𝖞𝖆𝖈𝖊_𝖒𝖆𝖐𝖊_𝖈𝖍𝖊𝖈𝖐_𝖛𝖆𝖗(𝓿𝓪𝓻_𝓷𝓪𝓶𝓮, 𝓯𝓲𝓮𝓵𝓭_𝓷𝓪𝓶𝓮, 𝓭𝓮𝓯𝓪𝓾𝓵𝓽_𝓿𝓪𝓵𝓾𝓮)` macro so the definition above
can be turned into the following:

~~~{.cc}
    𝖞𝖆𝖈𝖊_𝖒𝖆𝖐𝖊_𝖈𝖍𝖊𝖈𝖐_𝖛𝖆𝖗(𝔫𝔬𝔢𝔵𝔠𝔢𝔭𝔱, no_exceptions, false);
~~~

Now you can use `noexcept(::𝘆𝗮𝗰𝗲::𝔫𝔬𝔢𝔵𝔠𝔢𝔭𝔱<𝓸𝓹𝓽𝓲𝓸𝓷𝓼>)` to declare function which use `𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖊𝖖𝖚𝖆𝖑` or `𝖞𝖆𝖈𝖊_𝖈𝖍𝖊𝖈𝖐_𝖎𝖘_𝖎𝖓_𝖗𝖆𝖓𝖌𝖊`
sanity checks.

# Combining options # {#Combining_options}

Note that since different types and different modules support different options various non-trivial combinations of options are
possible.

To make sure we wouldn't face the problem of combinatorial explosions of types we provide template “combiner variable”
𝘆𝗮𝗰𝗲::𝔬𝔭𝔱𝔦𝔬𝔫𝔰.

It's definited like this:

~~~{.cc}
    template<auto... 𝓸𝓹𝓽𝓲𝓸𝓷𝓼>
    class 𝑜𝑝𝑡𝑖𝑜𝑛𝑠;

    template<>
    class 𝑜𝑝𝑡𝑖𝑜𝑛𝑠<> {
    };

    template<auto 𝓸𝓹𝓽𝓲𝓸𝓷>
    class 𝑜𝑝𝑡𝑖𝑜𝑛𝑠<𝓸𝓹𝓽𝓲𝓸𝓷> : public decltype(𝓸𝓹𝓽𝓲𝓸𝓷) {
     public:
      constexpr 𝑜𝑝𝑡𝑖𝑜𝑛𝑠() : decltype(𝓸𝓹𝓽𝓲𝓸𝓷)(𝓸𝓹𝓽𝓲𝓸𝓷) {}
    };

    template<auto* 𝓸𝓹𝓽𝓲𝓸𝓷>
    class 𝑜𝑝𝑡𝑖𝑜𝑛𝑠<𝓸𝓹𝓽𝓲𝓸𝓷> : public 𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(*𝓸𝓹𝓽𝓲𝓸𝓷)> {
     public:
      constexpr 𝑜𝑝𝑡𝑖𝑜𝑛𝑠() : 𝒓𝒆𝒎𝒐𝒗𝒆_𝒄𝒗𝒓𝒆𝒇<decltype(*𝓸𝓹𝓽𝓲𝓸𝓷)>(*𝓸𝓹𝓽𝓲𝓸𝓷) {}
    };

    template<auto 𝓸𝓹𝓽𝓲𝓸𝓷, auto... 𝓸𝓹𝓽𝓲𝓸𝓷𝓼>
    class 𝑜𝑝𝑡𝑖𝑜𝑛𝑠<𝓸𝓹𝓽𝓲𝓸𝓷, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼...> : public 𝑜𝑝𝑡𝑖𝑜𝑛𝑠<𝓸𝓹𝓽𝓲𝓸𝓷>, public 𝑜𝑝𝑡𝑖𝑜𝑛𝑠<𝓸𝓹𝓽𝓲𝓸𝓷𝓼...> {
    };

    template<auto 𝓸𝓹𝓽𝓲𝓸𝓷, auto... 𝓸𝓹𝓽𝓲𝓸𝓷𝓼>
    inline constexpr 𝑜𝑝𝑡𝑖𝑜𝑛𝑠<𝓸𝓹𝓽𝓲𝓸𝓷, 𝓸𝓹𝓽𝓲𝓸𝓷𝓼...> 𝔬𝔭𝔱𝔦𝔬𝔫𝔰;
~~~

Because that's global variable it's both C++17 and C++20 compatible and you can use it in the following way:

~~~{.cc}
    𝑥86_𝑡𝑜_𝑟𝑖𝑠𝑐𝑣_𝑡𝑟𝑎𝑛𝑠𝑙𝑎𝑡𝑜𝑟<&::𝘆𝗮𝗰𝗲::𝔬𝔭𝔱𝔦𝔬𝔫𝔰<
        &::𝘆𝗮𝗰𝗲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝔞𝔰𝔰𝔢𝔯𝔱,
        &::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝔵86_16,
        &::𝘆𝗮𝗰𝗲::𝗿𝗶𝘀𝗰𝘃::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝔲𝔫𝔠𝔬𝔪𝔭𝔯𝔢𝔰𝔰𝔢𝔡>> translator1;
~~~

One caveat, though: if you would decide to declare a configuration variable (which is often convenient to do), e.g.

~~~{.cc}
    inline constexpr auto 𝔱𝔯𝔞𝔫𝔰𝔩𝔞𝔱𝔬𝔯_𝔬𝔭𝔱𝔦𝔬𝔫𝔰 = &::𝘆𝗮𝗰𝗲::𝔬𝔭𝔱𝔦𝔬𝔫𝔰<
        &::𝘆𝗮𝗰𝗲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝔞𝔰𝔰𝔢𝔯𝔱,
        &::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝔵86_16,
        &::𝘆𝗮𝗰𝗲::𝗿𝗶𝘀𝗰𝘃::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝔲𝔫𝔠𝔬𝔪𝔭𝔯𝔢𝔰𝔰𝔢𝔡>;
~~~

then you can declare `translator2` like this:

~~~{.cc}
    𝑥86_𝑡𝑜_𝑟𝑖𝑠𝑐𝑣_𝑡𝑟𝑎𝑛𝑠𝑙𝑎𝑡𝑜𝑟<&𝔱𝔯𝔞𝔫𝔰𝔩𝔞𝔱𝔬𝔯_𝔬𝔭𝔱𝔦𝔬𝔫𝔰> translator2;
~~~

But then `translator1` and `translator2` would be a variables of different, and probably incompatible, types!
That's not a problem with C++20 style, though:

~~~
    inline constexpr auto 𝔱𝔯𝔞𝔫𝔰𝔩𝔞𝔱𝔬𝔯_𝔬𝔭𝔱𝔦𝔬𝔫𝔰 = ::𝘆𝗮𝗰𝗲::𝔬𝔭𝔱𝔦𝔬𝔫𝔰<
        ::𝘆𝗮𝗰𝗲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝔞𝔰𝔰𝔢𝔯𝔱,
        ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝔵86_16,
        ::𝘆𝗮𝗰𝗲::𝗿𝗶𝘀𝗰𝘃::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝔲𝔫𝔠𝔬𝔪𝔭𝔯𝔢𝔰𝔰𝔢𝔡>;

    𝑥86_𝑡𝑜_𝑟𝑖𝑠𝑐𝑣_𝑡𝑟𝑎𝑛𝑠𝑙𝑎𝑡𝑜𝑟<::𝘆𝗮𝗰𝗲::𝔬𝔭𝔱𝔦𝔬𝔫𝔰<
        ::𝘆𝗮𝗰𝗲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝔞𝔰𝔰𝔢𝔯𝔱,
        ::𝘆𝗮𝗰𝗲::𝘅𝟴𝟲::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝔵86_16,
        ::𝘆𝗮𝗰𝗲::𝗿𝗶𝘀𝗰𝘃::𝐨𝐩𝐭𝐢𝐨𝐧𝐬::𝔲𝔫𝔠𝔬𝔪𝔭𝔯𝔢𝔰𝔰𝔢𝔡>> translator3;

    𝑥86_𝑡𝑜_𝑟𝑖𝑠𝑐𝑣_𝑡𝑟𝑎𝑛𝑠𝑙𝑎𝑡𝑜𝑟<𝔱𝔯𝔞𝔫𝔰𝔩𝔞𝔱𝔬𝔯_𝔬𝔭𝔱𝔦𝔬𝔫𝔰> translator4;
~~~

Here `translator3` and `translator4` have the same type. Note that `translator1` and `translator2` have different types even in
C++20.
