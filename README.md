argon2_rs
=========

argon2_rs provides password hashing in Python using the argon2 algorithm, 
backed by  the Rust `argon2` crate. It is a drop-in replacement for the
high-level functionality in the argon2-cffi library.

This project was developed as a learning exercise, and at this time, is not
actively maintained (this may change in the future of course). It is 
intentionally not published on PyPI. Also it's missing a bunch of basic things
like error handling, tests and documentation.

## Basic usage
```python
import argon2_rs as argon2

ph = argon2.PasswordHasher()

# Create a password hash
hash_ = ph.hash("hunter2")
print(hash_)
# "$argon2id$v=19$m=19456,t=2,p=1$nEpj9F8SoHWXTlhHXfrrFw$Bul2k/ZIsn42nmXH+JBL7ouhmPnJzeupzPFN+bEl6vQ"

# Verify user input against a hash
ph.verify(hash_, "hunter2")
# True

# Check if the stored hash should be re-hashed
ph.check_needs_rehash(hash_)
# False
```

## Why?

This was a learning exercise to see if I could write a replacement for
argon2-cffi in just Rust. The answer seems to be yes!

## Differences from argon2-cffi

* Only the high-level functions shown above are supported.
* Defaults parameters are different (this uses the argon2 crate's defaults):
  * memory cost: 19,456 (argon2_rs) vs 65,536 (argon2-cffi)
  * time cost: 2 vs 3
  * parallelism: 1 vs 4
* Salt length is not configurable and fixed at 16.
* Only UTF-8 strings are supported for input.

## Is this library better than argon2-cffi?

Maybe!

The C libargon2 code powering argon2-cffi is unlikely to have major memory
safety vulnerabilities, but it does appear to be unmaintained or neglected,
with the last commit in 2021. It is likely incredibly widely used and has
probably received many more eyeballs looking at it.

The Rust argon2 crate is a port of the C code, so any logic bugs
could've been ported over. It appears to be actively maintained in the 
RustCrypto organization.

Depending on your architecture and CPU features, the C code might be faster
because of better optimizations and SIMD support. Presumably the Rust code
will eventually reach parity in this regard (e.g. AVX2 SIMD support was 
[recently added](https://github.com/RustCrypto/password-hashes/pull/440)).
There's also going to be some overhead from Python<-->C and Python<-->Rust
but I don't know about the performance characteristics of either.

But it's also a chicken-and-egg situation, people don't have an incentive
to improve the Rust version if people aren't using it, but people won't use
it if it isn't better.

## License

Available under the GPL, v3 or (at your option) any later version. See COPYING
for more details.

(C) 2023 Freedom of the Press Foundation & Kunal Mehta
