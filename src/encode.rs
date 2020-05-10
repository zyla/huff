use crate::tree;
use crate::tree::Code;

const WORD_SIZE_IN_BITS: usize = 64;

/// Encode a sequence of symbols using the given code.
pub fn encode(code: &Code, input: &[u8]) -> Vec<u64> {
    // Buffer of bits to output.
    let mut buf: u64 = 0;
    // Number of bits in buffer.
    let mut offset: usize = 0;

    let mut output: Vec<u64> = vec![];

    for &sym in input {
        let cw = &code[sym];

        // Number of full words we'll copy.
        // Note: (W * num_words) may be larger than number of full words of codeword.
        // This counts codeword bits and leftovers in the buffer together.
        let num_words = (offset + cw.bit_len) / WORD_SIZE_IN_BITS;

        // Copy all full words.
        for w in 0..num_words {
            output.push(buf | (cw.bits[w] << offset));
            buf = cw.bits[w] >> (WORD_SIZE_IN_BITS - offset);
        }

        // At this point, we know:
        // - we have consumed num_words*W bits of codeword.
        // - we know that (cw->len - num_words*W) < W. (Proof?)
        // So we can copy one last part of the codeword, into the partially filled buffer.
        buf |= cw.bits[num_words] << offset;

        // Shift the offset by codeword len.
        offset = (offset + cw.bit_len) % WORD_SIZE_IN_BITS;
    }

    // One last (partial) output word.
    if offset > 0 {
        output.push(buf);
    }

    output
}

/// Build code for input and encode it using the code.
pub fn full_encode(input: &[u8]) -> (Code, Vec<u64>) {
    let code = tree::tree_to_code(&tree::build_tree(&tree::compute_frequencies(input)));
    let output = encode(&code, input);
    (code, output)
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
