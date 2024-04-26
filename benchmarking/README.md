# Benchmarking Tools

This directory contains tools for benchmarking the performance of the library.

- /geneticAlgorithm: Contains all 17 outputs from the CLI tool for the genetic algorithm, and a Jupyter notebook for visualizing the results.
- /lichessDataset: Contains the pickled dataset of 100,000 games benchmarked (sourced from the Lichess December 2017 PGN Database) against our 4 compression strategies. Used for generating the graphs in the report.
- /entropy.ipynb: Jupyter notebook for visualizing the entropy of the Lichess December 2017 PGN Database. It not only calculates the lower bound entropy of the dataset, but also draws the example graphs for our Dynamic Huffman strategy.

Note: The actual dataset is not included due to its size. It is available for download from the Lichess website: https://database.lichess.org/ (December 2017 PGN Database).