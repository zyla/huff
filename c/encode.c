#include <stdio.h>
#include <stdint.h>
#include <unistd.h>
#include <string.h>

// A codeword can have a maximum of 256 bits.
// This pathological situation can arise when the Huffman tree is essentially a linked list.
#define MAX_CODEWORD_BITS 256

typedef uint8_t word_t;

// Word size in bits.
#define W (sizeof(word_t) * 8)

typedef struct {
  size_t len; // Length in bits
  word_t bits[MAX_CODEWORD_BITS / W];
} codeword_t;

typedef codeword_t code_t[256];

void putword(word_t word) {
  fwrite(&word, sizeof(word), 1, stdout);
}

void encode(code_t code, char *input, size_t len) {
  word_t buf = 0;
  int offset = 0;

  for(int i = 0; i < len; i++) {
    char sym = input[i];
//    fprintf(stderr, "Encoding sym 0x%02x %c\n", (uint8_t) sym, sym);
    codeword_t *cw = &code[sym];

    // Number of full words we'll copy.
    // Note: (W * num_words) may be larger than number of full words of codeword.
    // This counts codeword bits and leftovers in the buffer together.
    int num_words = (offset + cw->len) / W;

//    fprintf(stderr, "offset=%d cwlen=%d words: %d\n", offset, cw->len, num_words);

    // Copy all full words.
    for(int w = 0; w < num_words; w++) {
      putword(buf | (cw->bits[w] << offset));
      buf = cw->bits[w] >> (W - offset);
    }

    // At this point, we know:
    // - we have consumed num_words*W bits of codeword.
    // - we know that (cw->len - num_words*W) < W. (Proof?)
    // So we can copy one last part of the codeword, into the partially filled buffer.
    buf |= (cw->bits[num_words] << offset);

    // Shift the offset by codeword len.
    offset = (offset + cw->len) % W;
  }

  // One last (partial) output word.
  if(offset > 0) {
    putword(buf);
  }
}

#define bufsize 4096

int main() {
  code_t code;
  memset(code, 0, sizeof(code));

  code['A'].len = 1;
  code['B'].len = 2;
  code['C'].len = 3;
  code['D'].len = 4;
  code['E'].len = 5;
  code['F'].len = 5;
  code['A'].bits[0] = 0b00000001;
  code['B'].bits[0] = 0b00000010;
  code['C'].bits[0] = 0b00000000;
  code['D'].bits[0] = 0b00000100;
  code['E'].bits[0] = 0b00001100;
  code['F'].bits[0] = 0b00011100;

  ssize_t len;
  char buf[bufsize];

  while((len = read(0, buf, bufsize)) > 0) {
    encode(code, buf, len);
  }

  return 0;
}
