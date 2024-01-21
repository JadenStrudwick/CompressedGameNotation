import init, {
  opening_huffman_decompress_pgn_str,
  opening_huffman_compress_pgn_str,
} from './cgn.js';

// Function to convert a Uint8Array to a hexadecimal string
function toHexString(byteArray) {
  return Array.from(byteArray, (byte) =>
    ('0' + (byte & 0xff).toString(16)).slice(-2)
  ).join('');
}

// Function to convert a hexadecimal string to a Uint8Array
function toUint8Array(hexString) {
  const length = hexString.length;
  const uint8Array = new Uint8Array(length / 2);
  for (let i = 0; i < length; i += 2) {
    uint8Array[i / 2] = parseInt(hexString.substring(i, i + 2), 16);
  }
  return uint8Array;
}

// Get elements from DOM
const compressInput = document.getElementById('compress-input');
const compressCopyButton = document.getElementById('compress-copy-button');
const compressDownloadButton = document.getElementById(
  'compress-download-button'
);
const decompressInput = document.getElementById('decompress-input');
const decompressCopyButton = document.getElementById('decompress-copy-button');
const decompressDownloadButton = document.getElementById(
  'decompress-download-button'
);

// Handle drag and drop
compressInput.addEventListener('dragover', (e) => {
  e.preventDefault();
  e.stopPropagation();
});
decompressInput.addEventListener('dragover', (e) => {
  e.preventDefault();
  e.stopPropagation();
});

// Drop event handler
compressInput.addEventListener('drop', (e) => {
  e.preventDefault();
  e.stopPropagation();
  const file = e.dataTransfer.files[0];
  const reader = new FileReader();
  reader.onload = () => {
    compressInput.value = reader.result;
  };
  reader.readAsText(file);
});
decompressInput.addEventListener('drop', (e) => {
  e.preventDefault();
  e.stopPropagation();
  const file = e.dataTransfer.files[0];
  const reader = new FileReader();
  reader.onload = () => {
    decompressInput.value = toHexString(new Uint8Array(reader.result));
  };
  reader.readAsArrayBuffer(file);
});

// Handle mouse leave
compressCopyButton.addEventListener('mouseleave', () => {
  compressCopyButton.innerText = 'Copy Hex String';
});
compressDownloadButton.addEventListener('mouseleave', () => {
  compressDownloadButton.innerText = 'Download CGN File';
});
decompressCopyButton.addEventListener('mouseleave', () => {
  decompressCopyButton.innerText = 'Copy PGN String';
});
decompressDownloadButton.addEventListener('mouseleave', () => {
  decompressDownloadButton.innerText = 'Download PGN File';
});

async function run() {
  await init();

  // handle compress copy button
  compressCopyButton.addEventListener('click', () => {
    try {
      const compressed = opening_huffman_compress_pgn_str(compressInput.value);
      if (compressed.length === 0) {
        compressCopyButton.innerText = 'Invalid PGN String!';
        return;
      }
      navigator.clipboard.writeText(toHexString(compressed));
      compressCopyButton.innerText = 'Copied!';
    } catch (e) {
      console.log(e);
      compressCopyButton.innerText = 'Invalid PGN String!';
    }
  });

  // handle compress download button
  compressDownloadButton.addEventListener('click', () => {
    try {
      const compressed = opening_huffman_compress_pgn_str(compressInput.value);
      if (compressed.length === 0) {
        compressDownloadButton.innerText = 'Invalid PGN String!';
        return;
      }
      const blob = new Blob([compressed], { type: 'application/octet-stream' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = 'compressed.cgn';
      a.click();
      URL.revokeObjectURL(url);
      compressDownloadButton.innerText = 'Downloaded!';
    } catch (e) {
      console.log(e);
      compressDownloadButton.innerText = 'Invalid PGN String!';
    }

  });

  // handle decompress copy button
  decompressCopyButton.addEventListener('click', () => {
    try {
      const decompressed = opening_huffman_decompress_pgn_str(
        toUint8Array(decompressInput.value)
      );
      if (decompressed.length === 0) {
        decompressCopyButton.innerText = 'Invalid Hex String!';
        return;
      }
      navigator.clipboard.writeText(decompressed);
      decompressCopyButton.innerText = 'Copied!';
    } catch (e) {
      console.log(e);
      decompressCopyButton.innerText = 'Invalid Hex String!';
    }
  });

  // handle decompress download button
  decompressDownloadButton.addEventListener('click', () => {
    try {

      const decompressed = opening_huffman_decompress_pgn_str(
        toUint8Array(decompressInput.value)
      );
      console.log(decompressed);
      if (decompressed.length === 0) {
        decompressDownloadButton.innerText = 'Invalid Hex String!';
        return;
      }
      const blob = new Blob([decompressed], { type: 'text/plain' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = 'decompressed.pgn';
      a.click();
      URL.revokeObjectURL(url);
      decompressDownloadButton.innerText = 'Downloaded!';
    } catch (e) {
      console.log(e);
      decompressDownloadButton.innerText = 'Invalid Hex String!';
    }
  });
}
run();
