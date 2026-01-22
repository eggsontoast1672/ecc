# Eggs C Compiler

This is my attempt at a C compiler. Since I am not smart enough (yet) to
implement it all off the dome, I am following along with [*Writing a C Compiler*][1]
by Nora Sandler. It is an excellent resource, and by no means does it just give
you the implementation. It walks you through a good sequence of steps to take
to write the compiler without handing you the answer, which I find very fun. If
you are interested, definitely give it a go!

[1]: https://norasandler.com/2017/11/29/Write-a-Compiler.html

This compiler targets `x86_64` assembly, sorry arm/powerpc/whatever else users.
In the event that I implement some minimal subset of the standard library, I
only plan to support POSIX compliant systems, so sorry Windows users.

## Upcoming Plans

- [ ] Improve the error output
    - [ ] Improve the error messages
    - [ ] Display file, line, and column information
    - [ ] Add pretty colors
- [ ] Write some tests
    - [ ] Doctests
    - [ ] Unit tests
    - [ ] Integration tests
- [ ] Improve documentation
    - [ ] Explain *why*, not *what*
    - [ ] Add comments for **all** public items!
    - [ ] ...and all private items too, if I feel like it
- [X] Clean up whatever is going on in `lib.rs`
- [ ] Allow for `utf8` source code
- [ ] Turn `ecc::lexer::Lexer` into an iterator
