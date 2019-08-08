# tetrs
Yet another TDD exploration in the context of game development using Tetris as an example

In this, now finished, project I explore outside-in TDD in the context of a well known game design.
Outside-in means that a test makes minimal assumptions about the architecture of the code, and it only cares
about the output. This leads to interesting dynamics when writing the code. I tend to write really crappy code
with this approach at first. It is, however, almost a nobrainer to refactor since the tests are so independent 
of any internals. It's very enjoyable to completely redo the architecture of the code, alomst behind the back of
the tests, which will still tell you if you have screwed up functionally. If you refactor the crap out of your code,
and at the same time you don't have to change any tests, you win.

The outside-in testing is achived by having a trait called TRenderer and implementing two versions of it:
- ToStringRenderer renders to strings, which tests can then make assertions about
- RaylibRenderer renders the game to an actual window using the excellent [Rust bindings](https://github.com/deltaphc/raylib-rs)
for the excellent [Raylib](https://www.raylib.com/) graphics library.

The project also demonstrates a variant of the "Object Mother" pattern, with the struct TestableGame and its attendant
TestableGameFactory. It is used througout the tests to configure the game and its dependencies to fit a given test, enabling
easy writing and subsequent reading of tests with complex setup.

The game itself is playable but not pretty.
TDD'ing a pretty game is probably not possible. But you can TDD an ugly one and make it easy for another, more graphically inclined person,
to make it pretty.
Maye next time!

PS: Seriouly though, Raylib is AWESOME.

