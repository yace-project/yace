𝔜𝔄ℭ𝔈 ownership passing
======================

[C++ Core Guidelines](https://isocpp.github.io/CppCoreGuidelines/CppCoreGuidelines) describe in many fine details how ownership of
objects is supposed to be handled in Modern C++. And most suggestions are very sensible and reasonable. They even work reasonably
well with Visual Studio.

Unfortunately many existing plaforms (Android, GNU/Linux, iOS, MacOS) use
[certain different C++ ABI](https://itanium-cxx-abi.github.io/cxx-abi/abi.html) — and there use of the `std::unique_ptr` is not as
suitable.

What is the problem? Passing of value paramaters
[non-trivial for the purposes of calls](https://itanium-cxx-abi.github.io/cxx-abi/abi.html#value-parameter) (including
`std::unique_ptr`) are passed **by reference** — and then it's destructor is called from caller.

Why is that a problem? Let's conside a very simple example:

~~~{.cc}
    class 𝐰𝐢𝐝𝐠𝐞𝐭 {
     public:
      𝐰𝐢𝐝𝐠𝐞𝐭();
      ~𝐰𝐢𝐝𝐠𝐞𝐭();
      void do_something();
    };

    class 𝐡𝐨𝐥𝐝𝐞𝐫 {
     public:
      𝐡𝐨𝐥𝐝𝐞𝐫(std::unique_ptr<𝐰𝐢𝐝𝐠𝐞𝐭> widget) : 𝗐𝗂𝖽𝗀𝖾𝗍(widget.release()) {
      }
     private:
      std::unique_ptr<𝐰𝐢𝐝𝐠𝐞𝐭> 𝗐𝗂𝖽𝗀𝖾𝗍;
    };

    𝐡𝐨𝐥𝐝𝐞𝐫 make_holder(std::unique_ptr<𝐰𝐢𝐝𝐠𝐞𝐭> widget) {
      return 𝐡𝐨𝐥𝐝𝐞𝐫(std::move(widget));
    }
~~~

Here we just pass ownership from one object to another. There are no change to produce exceptions, memory allocations and so on.
And, indeed,
[code produced](https://godbolt.org/#z:OYLghAFBqd5QCxAYwPYBMCmBRdBLAF1QCcAaPECAM1QDsCBlZAQwBtMQBGAFlICsupVs1qhkAUgBMAISnTSAZ0ztkBPHUqZa6AMKpWAVwC2tQVvQAZPLUwA5YwCNMxEJM6kADqgWF1tPYYmgl4%2BanRWNvZGTi5uisqYqn4MBMzEBAHGpu5KKmG0KWkEEXaOziAAzIqp6ZlBOTXF1qXR5RUAlIqoBsTIHADkUhXWyIZYANTiFTpGmEYkAJ5T2OIADACCa%2BujzAoKkwBiq%2BIAnAAip8fSx0enFyfH65Lit%2BeXdy/Hbw/i6zdf9y%2BK1egN%2BZ0mAHZZBtxh4DA5WHhkCAtuNDgD3tdPh8fk9sd8MSD3n98aCTsCMbizhB2lNoes0QA/UmY/44x7PIk/N5cx5sgmnCnssE0umo8YAN1QeHQ43QqAA%2BgpULMCAhrMBRRV6eIIRdtVstjs9ujhetOCzcQAOS2PACsto%2BvMFjvW0kh9Nh8MRyPFrotzvWNsDDudPMpQNd0ggCgI6BAIAMtDwAEcDJgFR4CMQpjpHVjA5yI07iySw0KBet9dhxgB3GXATAEdrjECm74Q95cztWos4gBs4mkF274it/Pug%2BkFfuPep9fQjYIADpiAldpgaS3dZ7IRcYR5iHgJcwCBxxbH44nk2mM1mc9N8xPiX2BeGzc/uTPidX27Ou5SPavpOQ4joBY6flO34nHOYobLq%2Bo6vBgYBqWwalqGxbvm%2B0GPO6RjMAA1hmCD6FgxAxnGCZJqm6aZtmuZPq6wHvNhoJllhuFgssdYNk225QuKa4ED0tB/sSqFmuhZqYcKbGsVxbqUVe8wSpuC5Lu0tIGvBepbP0nSsCA/R2v0pCmP0qxmagxl5jIcjjMqPR9JMkgVJwZkEMZVlaaQCCYMw5GUJ0hEgHaqxCMZ3BmUYYURRZVmkDZ/RmQoIARV5lkGaQcCwEgaBGB4eDsGQFAQAVRUlSgwiiJwqyrO4VDFWexBpRADjeWZDjWGkCzGR5pAFbM9AAPK0KwfVZaQWAEaI7CddNeBrkkalpVNmAAB6JAYZ79WZ1hnkZU2Ig4xC9XoWB7aQ2Z4LF/QeZ0ND0EwbAcDw/CCDVYhyHIQh4A4aWQJ0qBZn4a0ALQjZI4zg5eUxnBI9kyJwEKpQkSQaBA5h1NkpDmCUUQxME3i%2BHQOPE6EfgE2ULg5Oj%2BSFLU%2BhZIIuSJAzjTU60tPVEU5MNEUXNE5wnROb0b2GcZpnmQtyUbVa/bg/23DjDsojjHVy6rMunDjBAuCECQrnuaQ4x6IVxXOMbItmz9MieZ1vn%2BYF5Q0qQoXhZF/TRTLU3Jal6XXY7kv9JIMVcPVvuJf7QdZb5amtX4IDcEAA)
does precisely that.

But what would happen if we would move constructor from header (where it can be observed by a compiler) to a .cc file?
[Now code is no longer simple or short](https://godbolt.org/#z:OYLghAFBqd5QCxAYwPYBMCmBRdBLAF1QCcAaPECAM1QDsCBlZAQwBtMQBGAFlICsupVs1qhkAUgBMAISnTSAZ0ztkBPHUqZa6AMKpWAVwC2tEAFZeW9ABk8tTADljAI0zEuk0gAdUCwuto9QxNzXh8/NTpbeycjV3dOTyUVSNoGAmZiAiDjUwtFZUxVAPTMgmjHFzcPRQysnJD8hTryu0q46sSASkVUA2JkDgByKQBmO2RDLABqcVGdI0wjEgBPOexxAAYAQS3tyeYFBVmAMU3xAE4AEUvz6XOzy5uL8%2B3JcUfr26eP86%2BX8TbB5/Z5/DafUGAq6zADssh20y8BmcrDwyBAe2mpxB33uvx%2BALe%2BP%2BOIh3yBxMhF3BOMJVwgXTm8O2WIAfpTccCCa93mSAV8%2Ba8uSTLjTuVCGUzMdMAG6oPDoaboVAAfQUqEWBAQdmAktGzPEMJu%2Br2ewOR2x4u2nA5hIAHLbXmZHT9BaKXdtpLDmYjkaj0dKPTa3dsHSHnW6BbSwR7pBBmugQCADLQ8ABHAyYFVeAjEOY6F14kO86Ou0sUyNikXbY3YaYAdwVwEwBEZJoRXmIeBlzAIHGlCaTKfTmezufzheFkKJkZuIan32pHtrlv%2BMO%2BfPXdpLBIAbOJpHPaVuFxd99Iq88t8aDUapTtTSHg%2BWw%2BWI6Wo%2BKlyGvUZmABrLMEH0LBiHjAhE2TVMMyzHM83mScPR3EVP2rU8YxDFdG3QZtW29aViBbfpaFXadnytV8rXfL8jy/S9yTjQcQGWGVMAgbDcK6NtbxuHYhh6VgQCGMwhlIUwhk2UTUCEgsZDkaZ1X6QZZkkUZOFEgghMkrjSAQTBmFAygen/cxNiEITuFEoxTLErSpKE0SFBAMzNIk/jSDgWAkDQIwvDwdgyAoCAfL8gKUGEUROE2TZOFIKh/L7YgnIgZw7NIZw7EyFYhPU0gfMWegAHlaFYbK3NILA/1Edg0vwQjilYpzyswAAPIoDD7HLRLsPtBPK1FnGILK9CwLrSFzPBrKGdSehoegmDYDgeH4QQIrEOQ5CEPBnCcyAelQHMAiagBaQrJGmY6EzmK4JDkmROBhRzCmKDQICsBpTFiqwKlieJBHCfw6A%2B/7fEB2gfqqBIChSEoWmB2LkiKVJSiyCGOih5oynh2oyjRv7OB6RSBiWgShJE2zyukoYWrtXdjt3bhpgOURpiigA6TY2c4aYIFwQgSBUtTSGmPRfP8txBYJkWNpkDS7J0vSDOqBlSBMswzL6yyKck0gqcc5zxvl0mhkkKyuGi7X7KGOW3J01ikoCEBuCAA%3D%3D%3D)
Suddely it does lots of checks and two calls to destructor… why something “simple”… is not simple at all?

Well… C++ Core Guidelines may say
[take a unique_ptr<widget> parameter to express that a function assumes ownership of a widget](https://isocpp.github.io/CppCoreGuidelines/CppCoreGuidelines#Rr-uniqueptrparam)…
but that doesn't mean that compiler would know it! From C++ language POV function is not obligied to take ownership of object!
It may just ignore that argument — and then, according to
[Itanium C++ ABI](https://itanium-cxx-abi.github.io/cxx-abi/abi.html#value-parameter) function make_holder is supposed to do the
cleanup, call the destructor and free the allocated memory.

And that happens literaly on each call to each function (except when compiler can do full inlining). That… is not efficient.
Can we do something with that?

Yes — and we don't even need to violate it significantly. Instead of passing ownership with `std::unique_ptr` we can just use
`𝘆𝗮𝗰𝗲::𝒐𝒘𝒏𝒆𝒓` alias.
[Now generated code is simple](https://godbolt.org/#z:OYLghAFBqd5QCxAYwPYBMCmBRdBLAF1QCcAaPECAM1QDsCBlZAQwBtMQBGAFlICsupVs1qhkAUgBMAISnTSAZ0ztkBPHUqZa6AMKpWAVwC2tQVvQAZPLUwA5YwCNMxEJIDMpAA6oFhdbT1DE0FvXzU6Kxt7IycXd0VlTFV/BgJmYgJA41NOBJVw2lT0gki7R2dXDwU0jKzg3Ori0ujYyoBKRVQDYmQOAHIpN2tkQywAanE3HSNMIxIAT0nscQAGAEFB4dHMCamCec9MAH0CYmZCBSXVjfWCWc9hO92dfcPaZhmJgDEV8QBOAAi/zc4jWILcy3WBl8om%2Bv0B/0k/1%2BP3%2BQL%2BSL%2BAA5xKiERjxFivrj4eikViAGwktGI4G7AFjaroEAgLTMBzsI54KgnSY6JksvAKI7eax3YhHABufOp%2BJBYKWpDhNL%2B8vByzcsnW12uI2YCgUyvxv2kKNJyNBSLx6PN1otazNKvhyztf1%2BawZ4gA7Fq1mNPAYOXhkCBrmMjTbxKbZZG1lbzW6aa73Y7jf8XQn3QCIG1Jr7wwA/GMW6PJy3FxMIssOivO2ug7O5zVhsaS1B4dBjdCoI4KVAzAgIazAHN5nVeoHN7XrPUGiP2zj1tY46sAVnrVcz6aX0gmPrDAaDIZbS8X1ZXW7W67Lm6d2%2Br0ggFcxqbJ/wvKvJxNd5KpP%2BBMplqWl7xnet5pjWN4ZneHoAFRLGMADuHbAJgBBNvmnjEHgkrMHcobrOGAogAYtB4AAjgYxyeKcgFbsBMGgWm4Gxq%2BFp/NBEGTtg85/F6FquvxWJMeiVLSECgmEmxfxiZx6JCZOvreopOratWZ6Xh%2BEHXluLHsXJ9q7kYzAANbHAg%2BhYMQT7/omtlaW%2BRLPoSf4Jpi4I6PWDEQSJ7ESZe0l1tWALwRCSEoWhbR7vmYzEGh3S0Lx7oaTBDn2jpYH%2BWBBmJmsj7EXMkqYBAyHoKh6EYeOQLrH0HSsCAfSrn0pCmH0KzNagDWeTIciMl0PQ7IMnDNQQDXtW0HQIJgzBWZQHQmSAq4rEIDXcM1RiLctrXtaQnV9M1CggMto1tbVpBwLASBoEYnh4OwZAUBA123fdKDCKInArCsuRUHd4qHRADhjc1DjWOk8wNcNpDXTM9AAPK0KwEOnaQWDGaI7DA6jeBxckRWHSjmAAB5JAY%2BF9FDYrKFjrB4A4ZzEPMehYJDI3YRtFNnTQ9BMGwHA8PwgjvWIchyEIdOHZAHSoDR/gEwAtHDkhjPLTKTACEg9TInBegdiTJBoEDmHUOSkOYzTlC4uShH4dAmyEPi27QFsxBUDT6wURS1Po2SCEo%2BQpDUJTWGUrtW4oQf2w0Qcu60nAdH23S9FwdUNU1LVY3tROUvLFLcGMeqwp9AB0KzF5wYwQLghAkBM7i5GMeg3Xdzh1248eN6LMgjcDE2kFNM0VDmpALUtK19GtGco3tB1HaQJ3janfSSOtXBfVPO0z/PvcdEVxC%2BBo3BAA%3D%3D%3D)
— but is it safe?

Unfortunately the answer is *no*. It's not safe — and especially if exceptions are involved. In fact when exceptions are allowed
it becomes so unsafe that we turn `𝘆𝗮𝗰𝗲::𝒐𝒘𝒏𝒆𝒓` into `std::unique_ptr`. But even if there are no exceptions we are not out of hot
water. [RAII](https://en.wikipedia.org/wiki/Resource_acquisition_is_initialization) is good thing to use even if you are not using
exceptions. So how should we write `make_holder` function to minimize risk?

Here is [the proper way to do that](https://godbolt.org/#z:OYLghAFBqd5QCxAYwPYBMCmBRdBLAF1QCcAaPECAM1QDsCBlZAQwBtMQBGAFlICsupVs1qhkAUgBMAISnTSAZ0ztkBPHUqZa6AMKpWAVwC2tQVvQAZPLUwA5YwCNMxEADZSAB1QLC62nsMTQS8fNTorG3sjJxd3JRUw2gYCZmICAONTTkVlTFU/ZNSCCLtHZzdFFLSMoOyFKuLrUujy1wBKRVQDYmQOAHIpAGZrZEMsAGpxQZ0jTCMSAE8p7HEABgBBIZGxzEnpggWPTAB9AmJmQgVltc2NgjmPYXu9nQOj2mZZyYAxVfEATgAIgDBuJ1qDBisNgYfKIfn8gQDJAC/r8AcD/sj/gAOcRoxGY8TY754hEY5HY1yk9FIkF7QHjeroEAgLTMBzsY54KinKY6JksvAKY5eaz3YjHABufOpBNB4OWpHhNP%2B8ohK0Gsg2N1GzAUCmVBL%2B0lRZJRYOR%2BIxZqt5vWppVCJWtv%2Bf3WDPEAHYtetxh4DBy8MgQDdxobreITbKI%2BtLWbXTSXW6HUaAc7427ARA2lMfWGAH7R81RpMWosJxGl%2B3lp01sFZnOa0PjSWoPDocboVDHBSoWYEBDWYDZ3M3L3Apvaja6/Xhu2cOvrXFVgCsdcrGbTi%2Bkk29of9geDzcXC6ry836zXpY3jq3VekEHLWJT5IB55VFJJLopVO/IJlpYlheca3jeqbVte6a3u6ABUyzjAA7u2wCYAQjZ5h4xB4JKzD3CGGxhgKIAGLQeAAI4GCcHhnABm5AdBIGpmBMYvua/xQeBE7YHO/yeuaLp8dijEYlS0jAgJRKsf8okcRigkTj646jlOmxVqeF7vuBV6bsxbGyXaO5GMwADWJwIPoWDEI%2Bf4JjZmmvsST5Er%2B8ZYhCOh1vR4HCWx4kXlJtZVoCcGQjx3CSeWoLSD5qr1pFkZSfK%2BmxWJbS7nmjIEMyxGkRRVE0dMnkBWW15%2BdBxXsYuXGIchqEQER8ySpg1nxuF2KJZGMXymVcoJfF6zJVFgJtOhzbEKh3S0DxbrqdB9l2tpoE9RGlX3i1H78a5b5OY5f6Uk5sWFYBxWxuuy12hVyWZiF2AQEh6AoQQAB043sHqzUjaN2qesCGx9B0rAgH0K59KQph9KsoOoEDHkyHIjJdD0uxDJwoMEEDkMjaQCCYMwlmUB0xkgCuqxCED3Cg0YxOk%2BDkOkNDfSgwoICk%2BjEP/aQcCwEgaBGB4eDsGQFAQLz/OCygwiiJwqyrNkVAC%2BKzMQA4GOgw41ipAsQOo6QvOzPQADytCsFr7OkFgRmiOwqvm3g435E1zNm5gAAeeQGHhfQ62Kyg26weAOOcxALHoWDa2jWFU17HM0PQTBsBwPD8IIktiHIchCAHzOQB0qDUX4TsALQG5I4yF0yUyAhIcMyJwnpM7k%2BQaBA5g1FkpDmCUUQxME3i%2BHQbe96Efhd2ULh1I3iSFNU%2BiZII8R5FPDSjy04%2BVEUg91MvTTd%2BUnAdL23S9FwANAyDYM2wzLuUoXrjcOMupwtLT2rE9nDjBAuCECQkySIM2TjD0HzAWzhf7/zSrDWQMg0aqyxjjPG5RsykCJiTMmfQKYXzNgzJmLNSBs0xqfPokhKZcBlpgum2C8GwI6E1YgPgNDcCAA%3D%3D%3D):

~~~{.cc}
    𝐡𝐨𝐥𝐝𝐞𝐫 make_holder(𝒐𝒘𝒏𝒆𝒓<𝐰𝐢𝐝𝐠𝐞𝐭*> 𝔀𝓲𝓭𝓰𝓮𝓽) {
      std::unique_ptr<𝐰𝐢𝐝𝐠𝐞𝐭> widget(std::move(𝔀𝓲𝓭𝓰𝓮𝓽));
      return 𝐡𝐨𝐥𝐝𝐞𝐫(𝒐𝒘𝒏𝒆𝒓<𝐰𝐢𝐝𝐠𝐞𝐭*>(widget.release()));
    }
~~~

The first thing we do at the beginning of function is putting our “raw” pointer into `std::unique_ptr` “for safe keeping” and then
we can do what is needed — we wouldn't create a memory leak even if we would call function which itself generates exceptions.

And when we pass it to another function — we release it from confines of `std::unique_ptr` and pass it along.

Note: this
[works perfectly](https://godbolt.org/#z:OYLghAFBqd5QCxAYwPYBMCmBRdBLAF1QCcAaPECAM1QDsCBlZAQwBtMQBGAFlICsupVs1qhkAUgBMAISnTSAZ0ztkBPHUqZa6AMKpWAVwC2tQVvQAZPLUwA5YwCNMxEADZSAB1QLC62nsMTQS8fNTorG3sjJxd3JRUw2gYCZmICAONTTkVlTFU/ZNSCCLtHZzdFFLSMoOyFKuLrUujy1wBKRVQDYmQOAHIpAGZrZEMsAGpxQZ0jTCMSAE8p7HEABgBBIZGxzEnpggWPTAB9AmJmQgVltc2NgjmPYXu9nQOj2mZZyYAxVfEATgAIgDBuJ1qDBisNgYfKIfn8gQDJAC/r8AcD/sj/gAOcRoxGY8TY754hEY5HY1yk9FIkF7QHjeroEAgLTMBzsY54KinKY6JksvAKY5eaz3YjHABufOpBNB4OWpEZBGZIAMtDwAEcDCcPGc%2BQKQMQ5qhJbrUGLnLzprKMfKIStIddBrINjdRswFAp4TT/n9pKiySiwcj8Rig2Hg%2BtA76EStI36wQzxAB2V3rcYeAwcvDIEA3cY%2Bgn%2BmPFkO24OIhN/aMVxP/eNBxPrQEQNpTdOFgB%2BdZLvfL1Zpg9rg4b/ZbbY7BfGkot6HG6FQxwUqFmBAQ1mAk5dN1TwJ3bo2Hq9RfDYM449xw4ArP2q024%2BPpJM0wWszm89Pxxfh1eH2Db1HYFR0bWMwWkCA6yxUtyQBP9fQpEkEwpKlkJBGVBwDcdQ3/e8wJHXDQLLFsACplnGAB3PB0GATACHbA8Mw8Yg8ElZh7nzDZC0NdUtR1EV9RtTCYKjHCwLw4iRPrIizxbci6xTYME0U7ExIJKlpGApsVKkjSZIBFT93TPcp0PTZhx/f91ng4jANwrTxP05tnyMZgAGsTgQfQsGISC0MTfybNg4koKJVCmyxCEdH7LDhzUs8JNkqTH2HQEyMhU8AW4IkpNBaR4vQuSEzy3KwScvLATaF9O2VVVeO1XVBOi4TsLvBzJLvJya33bBKOo2iCAgQ15jNPym2y7FSvyut5Xau1xFipt5XKhbKoYmrjQIbpaEy5tLPwoKozs8S5srLrwLGhClIiuDQpCtDKVC/4opi5KB3s8c3rHVL0uwCAqJouiADpjXYT1MDbNp1t3FNgQ2PoOlYEA%2BmvPpSFMPpVjR1BkeimQ5EZLoel2IZODRghkaxqHSAQTBmB8ygOjckBr1WIRke4NGjBZtmMax0gcb6NGFBANmKcxhHSDgWAkDQIwPDwdgyAoCA5YVpWUGEUROFWVZsioRXxRFiAHEptGHGsVIFmRsnSDl2Z6AAeVoVhrYl0gsFc0R2DNj28GNfIzRF93MAADzyAwOL6W3LSR93WDwBxzmIBY9CwG3yZY7no8lmh6CYNgOB4fhBC1sQ5DkIRE5FyAOlQPU/GDgBaR3JHGJumSmQEJHxmROBTYXcnyDQIHMGoslIcwSiiGJgm8Xw6HHufQj8aeyhcOoh8SQpqn0TJBHiPJt4aNeWg3yoiiXuoT6aGfyk4DoV26XouER5HUfR33BdDykm9cbhxgejhDrIGqwgacHGBAXAhASCTEkIMbI4w9Dy0Vs4OBCCqp41kDIcmZtqa03puUNspBmas3Zn0Tmn93aC2FqLUg4sqZvz6JILmXBdZUP5jQ%2BheCOhmmID4DQ3AgA)
even if `𝘆𝗮𝗰𝗲::𝒐𝒘𝒏𝒆𝒓` becomes an alias for `std::unique_ptr`.

But of course that's quite a lot of boilerplate. To make code more understandable we offer couple of macroses. With them the code
above would look like this:

~~~{.cc}
    𝐡𝐨𝐥𝐝𝐞𝐫 make_holder(𝒐𝒘𝒏𝒆𝒓<𝐰𝐢𝐝𝐠𝐞𝐭*> 𝔀𝓲𝓭𝓰𝓮𝓽) {
      𝖞𝖆𝖈𝖊_𝖍𝖔𝖑𝖉_𝖔𝖜𝖓𝖊𝖗_𝖕𝖆𝖗𝖆𝖒𝖆𝖙𝖊𝖗(widget, 𝔀𝓲𝓭𝓰𝓮𝓽);
      return 𝐡𝐨𝐥𝐝𝐞𝐫(𝖞𝖆𝖈𝖊_𝖕𝖆𝖘𝖘_𝖔𝖜𝖓𝖊𝖗_𝖆𝖗𝖌𝖚𝖒𝖊𝖓𝖙(widget));
    }
~~~

Note: 𝖞𝖆𝖈𝖊_𝖕𝖆𝖘𝖘_𝖔𝖜𝖓𝖊𝖗_𝖆𝖗𝖌𝖚𝖒𝖊𝖓𝖙 will usually accept 2nd, optional, argument, 𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼. These are used
([as usual in 𝔜𝔄ℭ𝔈](md_options_cxx17.html)) to handle difference 𝔜𝔄ℭ𝔈 options. If you want to pick fast owner passing (as described
above) then these options should include member `fast_owner` and it must be set to `true`. If there are no such explicit option,
then, by default, `owner` becomes fast if `𝔫𝔬𝔢𝔵𝔠𝔢𝔭𝔱<𝔂𝓪𝓬𝓮_𝓸𝓹𝓽𝓲𝓸𝓷𝓼>` is `true`.

