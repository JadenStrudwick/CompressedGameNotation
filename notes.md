# TODOS

## Features
- [ ] Only store the 7 tag roster information, not all headers. We only need the keys themselves, given the correct order.
- [ ] Ensure we are in "reduced export format" (see below)
- [ ] Implement a PGN iterator
- [ ] Create variants of compress/decompress that strip comments/extra headers
- [ ] Set up benchmarking utilities

## Misc
- [ ] Clean up comments, some start with captials inconsistently

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
