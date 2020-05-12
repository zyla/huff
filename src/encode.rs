use crate::tree::Code;

#[cfg(test)]
use crate::tree;

const WORD_SIZE_IN_BITS: usize = 64;

pub struct Encoder<'a> {
    code: &'a Code,
    /// Buffer of bits to output.
    buf: u64,
    /// Number of bits in buffer.
    offset: usize,
}

impl<'a> Encoder<'a> {
    pub fn new(code: &'a Code) -> Self {
        Encoder {
            code,
            buf: 0,
            offset: 0,
        }
    }

    /// Encode a chunk of the input to the given output buffer.
    /// Stops when either input is exhausted or buffer is full.
    /// Returns the number of input bytes consumed, and the number of u64 words filled in the buffer.
    pub fn encode(&mut self, input: &[u8], output: &mut [u64]) -> (usize, usize) {
        let mut num_output_words_written = 0;

        for (index, &sym) in input.iter().enumerate() {
            let cw = &self.code[sym];

            // Number of full words we'll copy.
            // Note: (W * num_words) may be larger than number of full words of codeword.
            // This counts codeword bits and leftovers in the buffer together.
            let num_words = (self.offset + cw.bit_len) / WORD_SIZE_IN_BITS;

            if num_output_words_written + num_words > output.len() {
                return (index, num_output_words_written);
            }

            // Copy all full words.
            for w in 0..num_words {
                output[num_output_words_written] = self.buf | (cw.bits[w] << self.offset);
                num_output_words_written += 1;
                self.buf = cw.bits[w] >> (WORD_SIZE_IN_BITS - self.offset);
            }

            // At this point, we know:
            // - we have consumed num_words*W bits of codeword.
            // - we know that (cw->len - num_words*W) < W. (Proof?)
            // So we can copy one last part of the codeword, into the partially filled buffer.
            self.buf |= cw.bits[num_words] << self.offset;

            // Shift the offset by codeword len.
            self.offset = (self.offset + cw.bit_len) % WORD_SIZE_IN_BITS;
        }

        (input.len(), num_output_words_written)
    }

    pub fn finish(&mut self, output: &mut [u64]) -> usize {
        // One last (partial) output word.
        if self.offset > 0 {
            output[0] = self.buf;
            self.offset = 0;
            self.buf = 0;
            1
        } else {
            0
        }
    }
}

/// Build code for input and encode it using the code.
#[cfg(test)]
pub fn full_encode(input: &[u8]) -> (Code, Vec<u64>) {
    let code = tree::tree_to_code(&tree::build_tree(&tree::compute_frequencies(input)));
    let mut encoder = Encoder::new(&code);
    let mut output: Vec<u64> = (0..input.len()).map(|_| 0).collect();
    let (input_consumed, mut output_consumed) = encoder.encode(input, &mut output);
    assert_eq!(input_consumed, input.len());
    output_consumed += encoder.finish(&mut output[output_consumed..]);
    (code, output[..output_consumed].to_vec())
}

#[cfg(test)]
fn bit_sequence_to_string(words: &[u64]) -> String {
    let mut output = String::new();
    for &w in words {
        let mut s = format!("{:b}", w);
        unsafe {
            s.as_mut_vec().reverse();
        }
        output.push_str(&s);
    }
    output
}

#[cfg(test)]
fn strip_indent(s: &'static str) -> String {
    let mut output = String::new();
    for line in s.lines() {
        let trimmed = &line.trim();
        if !trimmed.is_empty() {
            output.push_str(trimmed);
            output.push('\n');
        }
    }
    output
}

#[test]
fn test_full_encode() {
    let (code, output) = full_encode(b"appends_a_given_slice");
    println!("{}", &code);
    assert_eq!(
        format!("{}", &code),
        strip_indent(
            "
            _: 101
            a: 000
            c: 11110
            d: 11101
            e: 110
            g: 11111
            i: 011
            l: 1000
            n: 010
            p: 001
            s: 1001
            v: 11100
            "
        )
    );
    assert_eq!(
        bit_sequence_to_string(&output),
        "000001001110010111011001101000101111110111110011001010110011111111011".to_string()
    );
}
