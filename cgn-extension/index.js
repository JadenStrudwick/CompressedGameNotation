import init, {
  dynamic_huffman_decompress_pgn_str,
  dynamic_huffman_compress_pgn_str,
} from './cgn.js';

function toHexString(byteArray) {
  return Array.from(byteArray, function (byte) {
    return ('0' + (byte & 0xff).toString(16)).slice(-2);
  }).join('');
}

function hexStringToIntArray(hexString) {
  let intArray = [];
  for (let i = 0; i < hexString.length; i += 2) {
    intArray.push(parseInt(hexString.substr(i, 2), 16));
  }
  return intArray;
}

async function run() {
  await init();

  document.getElementById('compress-input').addEventListener('input', () => {
    let text = document.getElementById('compress-input').value;
    let compressed = dynamic_huffman_compress_pgn_str(text);
    let hex_string = toHexString(compressed);
    document.getElementById('compress-output').innerHTML = hex_string;
  });

  document.getElementById('decompress-input').addEventListener('input', () => {
    let text = document.getElementById('decompress-input').value;
    let intArray = hexStringToIntArray(text);
    let decompressed = dynamic_huffman_decompress_pgn_str(intArray);
    document.getElementById('decompress-output').innerHTML = decompressed;
    console.log(text, intArray, decompressed);
  });

  console.log('Done');
}
run();
