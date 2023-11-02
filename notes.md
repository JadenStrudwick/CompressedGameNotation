# TODOS

## Features
- [ ] Set up benchmarking utilities

## Misc
- [ ] Clean up comments, some start with captials inconsistently
- [ ] Add comments to all tests, explaining what they are testing

# 31st of October 2023

I am currently working on implementing an iterator for the PGN parser.
This is to get the benchmarking utilities set up, and allow us to open a 
large lichess database, parse, and compress it.

There is a slight issue. A lot of the lichess games contain comments.
We can include comment support, and I've done some preliminary experimentation with it.
However, the presence of comments in a PGN file seems counter intutive to our purposes of maximally compressing.

I am considering stripping the comments from the games. We have tests that show for games without comments, we are
able to perfectly recreate the file. So for games with comments, it stands that we can recreate the file, but with
the comments stripped.

PROS:
- We can compress the file more
- We can still recreate the file perfectly (with comments stripped)
- Comments are not important for our purposes

CONS:
- We lose the ability to recreate the file with comments (lossy compression)

So the dilemma comes down to whether we are willing to perform some lossy compression to get a better compression ratio.

** An idea would be to just have some flags that enable or disable comments/extra-headers. **

---

Also, I don't need to implement an iterator for the PGN parser/vistor myself. It is provided by the pgn_reader crate.
You just pass in the reader you need, then you can use it to iterate over the games.

However, this means I will not be able to see the original string of the game. This was important, as it is required
for accurately computing the compression ratio.

This leaves me with two options:
- Implement my own iterator
- Modify the PgnVisitor to store the original string of the game
  - This is difficult as there is no way to get the original string from the visitor

I think I will go with the first option, as it is the easiest to implement.

These are apparently the rules for a MINIMAL PGN in "reduced export format"
- it has no commentary
- it has only the standard seven tag roster identification information ("STR", see below)
- it has no recursive annotation variations ("RAV", see below) and
- it has no numeric annotation glyphs ("NAG", see below). Reduced export format is used for bulk storage of unannotated games. It represents a minimum level of standard conformance for a PGN exporting application.

Because of this, I will be ensuring we are compressing to "reduced export format".

## Key points from today
- I cannot use the pgn_reader iterator, as it does not give me the original string of the game

# 2nd of November 2023

I have implemented my own iterator for the PGN parser. It parses each game, and returns the original string of the game.
This is so we can accurately compute the compression ratio.

Also I have changed the PgnData struct to only support 'reduced export format' PGNs. This means we only store the 7 tag roster information,
no comments, no RAVs, no NAGs.

This means we will have "lossy" compression, but we still capture all the vital information about a game.

I have also just finished setting up the code coverage tooling with tarpaulin and codecov.io. This will allow us to see how much of the code is covered by tests. This is important for ensuring we have good test coverage.

Currently there is approx 75% code coverage. The main areas not covered are benchmark.rs and lib.rs, as those are still under active development.

