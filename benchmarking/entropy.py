from math import log2

def parse_rust_function_to_pairs(rust_function_str):
    pairs = []
    for line in rust_function_str.split("\n"):
        if "weights.insert" in line:
          pair = line.split("weights.insert")[1][1:-2].split(", ")
          pairs.append((int(pair[0]), int(pair[1])))
    return pairs

def calculate_probabilities(pairs):
    total_freq = sum(freq for _, freq in pairs)
    probabilities = [(index, freq / total_freq) for index, freq in pairs]
    return probabilities

def calculate_entropy(probabilities):
    probabilities = [(index, p) for index, p in probabilities if p > 0]
    entropy = -sum(p * log2(p) for _, p in probabilities)
    return entropy

# open the huffman_codes.rs file and read the contents
rust_function_str = open("cgn/src/compression/utils/huffman_codes.rs").read()
pairs = parse_rust_function_to_pairs(rust_function_str)
probabilities = calculate_probabilities(pairs)
entropy = calculate_entropy(probabilities)
print(entropy)
