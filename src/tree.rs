#![allow(clippy::ptr_arg)]

use crate::heap;
use crate::heap::keyed::Keyed;

pub type Frequencies = Vec<(u8, usize)>;

pub fn compute_frequencies(input: &[u8]) -> Frequencies {
    let mut table: Vec<usize> = (0..NUM_SYMBOLS).map(|_| 0).collect();
    for s in input {
        table[*s as usize] += 1;
    }
    table
        .into_iter()
        .enumerate()
        .filter(|(_, freq)| *freq > 0)
        .map(|(s, freq)| (s as u8, freq))
        .collect()
}

#[test]
fn test_compute_frequencies() {
    assert_eq!(
        compute_frequencies(b"ABCAAABABABC"),
        vec![(b'A', 6), (b'B', 4), (b'C', 2)]
    );
}

type Symbol = u8;

#[derive(PartialEq, Eq, Debug)]
pub enum Tree {
    Branch(Box<Tree>, Box<Tree>),
    Leaf(Symbol),
}

const NUM_SYMBOLS: usize = 256;

pub fn build_tree(frequencies: &Frequencies) -> Tree {
    let mut heap: Vec<Keyed<usize, Tree>> = Vec::with_capacity(frequencies.len());
    for (symbol, freq) in frequencies {
        heap::insert(&mut heap, Keyed::new(*freq, Tree::Leaf(*symbol)));
    }
    while let Some(left) = heap::pop(&mut heap) {
        if let Some(right) = heap::pop(&mut heap) {
            heap::insert(
                &mut heap,
                Keyed::new(
                    left.key + right.key,
                    Tree::Branch(Box::new(left.value), Box::new(right.value)),
                ),
            );
        } else {
            return left.value;
        }
    }
    panic!("No symbols with non-zero frequency in input");
}

#[test]
fn test_build_tree() {
    assert_eq!(
        build_tree(&vec![(b'A', 6), (b'B', 4), (b'C', 2)]),
        Tree::Branch(
            Box::new(Tree::Leaf(b'A')),
            Box::new(Tree::Branch(
                Box::new(Tree::Leaf(b'C')),
                Box::new(Tree::Leaf(b'B'))
            ))
        )
    );
}

pub type Code = Vec<Codeword>;

const MAX_CODEWORD_BITS: usize = NUM_SYMBOLS;
const NUM_CODEWORD_WORDS: usize = MAX_CODEWORD_BITS / 64;

#[derive(PartialEq, Eq, Clone)]
// Invariant: all bits after bit_len are 0
pub struct Codeword {
    pub bit_len: usize,
    pub bits: [u64; NUM_CODEWORD_WORDS],
}

impl Codeword {
    pub fn empty() -> Self {
        Codeword {
            bit_len: 0,
            bits: [0; NUM_CODEWORD_WORDS],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.bit_len == 0
    }

    pub fn push_bit(&mut self, bit: bool) {
        let index = self.bit_len;
        self.bits[index / 64] |= (bit as u64) << (index % 64);
        self.bit_len += 1;
    }

    pub fn pop_bit(&mut self) {
        let index = self.bit_len - 1;
        self.bits[index / 64] &= !(1 << (index % 64));
        self.bit_len -= 1;
    }
}

impl std::fmt::Debug for Codeword {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("[")?;
        let bit_values = ["0", "1"];
        for i in 0..self.bit_len {
            f.write_str(bit_values[((self.bits[i / 64] >> (i % 64)) & 1) as usize])?;
        }
        f.write_str("]")
    }
}

pub fn tree_to_code(tree: &Tree) -> Code {
    let mut code: Code = (0..NUM_SYMBOLS).map(|_| Codeword::empty()).collect();
    explore_tree(&mut code, &mut Codeword::empty(), tree);
    code
}

fn explore_tree(code: &mut Code, prefix: &mut Codeword, tree: &Tree) {
    match tree {
        Tree::Branch(left, right) => {
            prefix.push_bit(false);
            explore_tree(code, prefix, left);
            prefix.pop_bit();
            prefix.push_bit(true);
            explore_tree(code, prefix, right);
            prefix.pop_bit();
        }
        Tree::Leaf(symbol) => {
            code[*symbol as usize] = prefix.clone();
        }
    }
}

#[test]
fn test_tree_to_code() {
    let code = tree_to_code(&Tree::Branch(
        Box::new(Tree::Leaf(b'A')),
        Box::new(Tree::Branch(
            Box::new(Tree::Branch(
                Box::new(Tree::Leaf(b'C')),
                Box::new(Tree::Leaf(b'E')),
            )),
            Box::new(Tree::Leaf(b'B')),
        )),
    ));
    assert_eq!(format!("{:?}", code[b'A' as usize]), "[0]");
    assert_eq!(format!("{:?}", code[b'B' as usize]), "[11]");
    assert_eq!(format!("{:?}", code[b'C' as usize]), "[100]");
    assert_eq!(format!("{:?}", code[b'E' as usize]), "[101]");
}