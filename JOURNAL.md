# Compressed Game Notation
3rd Year Project for University of Warwick Computer Science

## 31st of October 2023

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

Because of this, I will be ensuring we are compressing to "reduced export format". I cannot use the pgn_reader iterator, as it does not give me the original string of the game

## 2nd of November 2023

I have implemented my own iterator for the PGN parser. It parses each game, and returns the original string of the game.
This is so we can accurately compute the compression ratio.

Also I have changed the PgnData struct to only support 'reduced export format' PGNs. This means we only store the 7 tag roster information,
no comments, no RAVs, no NAGs.

This means we will have "lossy" compression, but we still capture all the vital information about a game.

I have also just finished setting up the code coverage tooling with tarpaulin and codecov.io. This will allow us to see how much of the code is covered by tests. This is important for ensuring we have good test coverage.

Currently there is approx 75% code coverage. The main areas not covered are benchmark.rs and lib.rs, as those are still under active development.

I've made some progress on the benchmarking. Right now I am having issues with the bits per move calculations however. I believe this is due to some floating point error, or maybe some game messing with the calculations. I will have to investigate this further.

## 3rd of November 2023

I'm currently trying to set up the benchmarks, but I think I need to refactor the lib slightly.

I've move the iterator into the benchmark utilies, as it is custom to the LichessDB format, and not useful to include as an export in my library.

The utils also contain the metrics collection code required for the benchmarks.

I've also cleaned up the pgn_data module, moving the vistor and sanpluswrapper into the module.

## 21st of November 2023

I've been working on the new encoding scheme inspired by Lichess.

Major refactor decision is to use BitVec instead of byte arrays for my compressed data. This saves us from any wasted padding bytes.

I NEED TO CHECK MY COMMENTS OVER

## 4th of January 2024

Had a break over the holidays, and due to the CS324 game, but I am back now. I need to start working on the final compression scheme.

However, I spent some time today just refamiliarizing myself with the codebase, and fixing a few explict 'as' casts that could be replaced with 'try_into' casts.

## 11th of Janurary 2024

I have implemented the third strategy, but now need to find the optimal height and deviation for the gaussian. I have written a straightfoward genetic algorithm to try and find the optimal value. I have discovered from this first run that the optimal height is likely between 1 to 25M and the optimal deviation is between 1 and 6.

I am conducting another run of the GA with these new bounds.

Output 7 was made using 100 iterations, not 10 like all the previous. All further runs will be done with 100 iterations.

## 12th of Janurary 2024

Just had an interesting problem with dynamic huffman. Since the string variation of the compression uses padding bytes at the end, the decoder would try to decompress the padding bytes, and we had no way to tell it to stop. To fix this, I added a check where if the game is a checkmate or stalemate, we stop decoding. 

I also added clap and a nicer way to inferface with the library via the CLI.

## 14th of Janurary 2024

Given the clap interface, I have decided to slightly refactor the format in which the genetic algorithm outputs its results. This is unfortunate, as I have already run some previous outputs through the GA, but I think it is worth it. I will begin my going through the previous outputs, and writing down their max-min and number of games, to produce new configs that represent them.

- output.txt:
  - init_population: 100
  - number_of_games: 10
  - genrations: 1000
  - mutation_rate: 0.2
  - tournament_size: 2
  - height_min: 0.0
  - height_max: 250_000_000
  - deviation_min: 1
  - deviation_max: 10
  - input_db_path: "cgn/benches/lichessDB.pgn"
  - output_path: "cgn/output.txt"

- output3.txt:
  - init_population: 100
  - number_of_games: 10
  - genrations: 1000
  - mutation_rate: 0.2
  - tournament_size: 2
  - height_min: 0
  - height_max: 25_000_000
  - deviation_min: 1
  - deviation_max: 6
  - input_db_path: "cgn/benches/lichessDB.pgn"
  - output_path: "cgn/output3.txt"

- output4.txt:
  - init_population: 100
  - number_of_games: 10
  - genrations: 1000
  - mutation_rate: 0.2
  - tournament_size: 2
  - height_min: 0
  - height_max: 1_000_000
  - deviation_min: 1
  - deviation_max: 3
  - input_db_path: "cgn/benches/lichessDB.pgn"
  - output_path: "cgn/output4.txt"

- output5.txt:
  - init_population: 100
  - number_of_games: 10
  - genrations: 1000
  - mutation_rate: 0.2
  - tournament_size: 2
  - height_min: 800_000
  - height_max: 1_200_000
  - deviation_min: 2.5
  - deviation_max: 3.0
  - input_db_path: "cgn/benches/lichessDB.pgn"
  - output_path: "cgn/output5.txt"

- output6.txt:
  - init_population: 100
  - number_of_games: 10
  - genrations: 1000
  - mutation_rate: 0.2
  - tournament_size: 2
  - height_min: 900_000
  - height_max: 1_200_000
  - deviation_min: 2.5
  - deviation_max: 4.5
  - input_db_path: "cgn/benches/lichessDB.pgn"
  - output_path: "cgn/output6.txt"

- output7.txt:
  - init_population: 100
  - number_of_games: 100
  - genrations: 1000
  - mutation_rate: 0.2
  - tournament_size: 2
  - height_min: 700_000
  - height_max: 1_000_000
  - deviation_min: 2.3
  - deviation_max: 2.6
  - input_db_path: "cgn/benches/lichessDB.pgn"
  - output_path: "cgn/output7.txt"

- output8.txt:
  - init_population: 100
  - number_of_games: 100
  - genrations: 1000
  - mutation_rate: 0.2
  - tournament_size: 2
  - height_min: 700_000
  - height_max: 800_000
  - deviation_min: 2.55
  - deviation_max: 2.65
  - input_db_path: "cgn/benches/lichessDB.pgn"
  - output_path: "cgn/output8.txt"

- output9.txt:
  - init_population: 100
  - number_of_games: 100
  - genrations: 1000
  - mutation_rate: 0.2
  - tournament_size: 2
  - height_min: 720_000
  - height_max: 760_000
  - deviation_min: 2.50
  - deviation_max: 2.60
  - input_db_path: "cgn/benches/lichessDB.pgn"
  - output_path: "cgn/output9.txt"

- output10.txt
  - init_population: 100
  - number_of_games: 100
  - genrations: 1000
  - mutation_rate: 0.2
  - tournament_size: 2
  - height_min: 740_000
  - height_max: 760_000
  - deviation_min: 2.52
  - deviation_max: 2.60
  - input_db_path: "cgn/benches/lichessDB.pgn"
  - output_path: "cgn/output9.txt"

I've now deleted these output files, but these above recordings can be used to generate a new set of outputs for the final report.

## 20th of Janurary 2024

After meeting with my supervisor, he gave the good idea into looking into compressing the opening sequence of moves into a smaller representation. I implented this idea as 'opening huffman' and it actually beats dynamic huffman as well. This is very promising.

## 21st of Janurary 2024

I tried out compressing the results after they are converted to bitvecs using flate2. This doesn't actually work well at all, as flate2 requires you to give it bytes, so converting a bitvec to bytes may add a bunch of padding 0's. This is not ideal and when I roughly tested it on 'opening huffman' it actually made the bits per move around 8, roughly twice as bad. As such, I won't be implementing this idea.

## 28th of Janurary 2024

Been doing some reading into AC and ANS codings, after reading a old blog post about chess compression. I started using the constriction library and I got everything working, but I just found out that the encoded data is always padded to a 'word' size. This is not ideal, as it'll increase our compression ratios significantly. I was using a ANS coder, but now I need to find some other library that allows me to access the data without any data padding. This is quite unfortunate.

I just finished checking out Arcode, and it too uses the Write trait, which works on the byte level. 

Constriction was my best bet, as it provides ways to customise the Coders/Decoders significantly. However, when I looked into changing the coders word size to a boolean (so that it's output is a bitvec), I found that the Word requires the implementation of the BitArray traight. This seems to be very complex to implement, with the library author making exessive usage of UNSAFE and MACROS to implement them. I don't think I will be able to implement this trait for bools, so constriction is out the picture too.

I think this leaves me with two options:
1) Implement my own ANS/Range/Ac coder
2) Do not attempt this technique

It seems very difficult to implement a ANS coder, for possibly no gain. I think I will not attempt this technique.

## 3rd of Feburary 2024

Need to optimise the opening huffman encoding further.

1) See what Openings are most common (seems like only 512 openings can cover ~90% of games, which means 9 bits per opening)
2) Add code that allows for the number of openings to be changed (1-12)

I think I will use the dataset of openings and import that back into the library. So the tsvs yields the common openings, and then I use those instead of all the tsvs.

I just finished the refactor to use the txt file. I now need a way to using the CLI to change the parameters of the opening huffman benchmarks. After that I also need to go back and refactor the opening huffman stuff to make sure its all clear. Maybe remove the tsv files and intermediate python stuff I did (after at least commiting it once?)

- [x] Get CLI working and collect some data on it (find optimal 2 parameters)
- [x] Refactor opening huffman to make it clear
- [x] Remove tsv files and python stuff
- [x] Return benchmark.rs to its previous state (i.e. ensure all strategies get benchmarked)

## 19th of Feburary 2024

I've calculated the entropy limit using the Lichess frequencies and found it to be 4.381559249939818. This is pretty interesting, as it means that we're quite close to the limit already with Huffman-type encoding. I could look into Range Coding or ANS, but since all the libraries I find padded the output to a word size, I don't think its worth it in this case. 

If we had millions of moves in a game, it would be worth it, as any padding would be negligible even with a tiny reduction in bits per move. However, since there is approx 40 moves in a game, such padding is significant to the final file sizes.