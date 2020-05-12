use std::io::{Read, Write};
use std::time::Instant;
use std::{env, fs, io};

use huff::tree;

fn main() -> io::Result<()> {
    let args: Vec<_> = env::args().collect();

    let start = Instant::now();

    let mut input_buf: Vec<u8> = (0..8192).map(|_| 0).collect();

    let mut table: Vec<usize> = (0..256).map(|_| 0).collect();
    let mut infile = fs::File::open(&args[1])?;
    loop {
        let input_len = infile.read(&mut input_buf)?;
        if input_len == 0 {
            break;
        }
        for &s in input_buf[0..input_len].iter() {
            table[s as usize] += 1;
        }
    }
    let frequencies = table
        .into_iter()
        .enumerate()
        .filter(|(_, freq)| *freq > 0)
        .map(|(s, freq)| (s as u8, freq))
        .collect();

    eprintln!("T: counting:      {:?}", start.elapsed());

    let start = Instant::now();
    let code = tree::tree_to_code(&tree::build_tree(&frequencies));
    eprintln!("T: building code: {:?}", start.elapsed());

    let start = Instant::now();

    let mut output_buf: Vec<u64> = (0..8192).map(|_| 0).collect();
    let mut encoder = huff::encode::Encoder::new(&code);
    let stdout = io::stdout();
    let mut output = stdout.lock();
    let mut infile = fs::File::open(&args[1])?;
    loop {
        let input_len = infile.read(&mut input_buf)?;
        if input_len == 0 {
            break;
        }
        let mut input_off = 0;
        while input_off < input_len {
            let (input_consumed, output_len) =
                encoder.encode(&input_buf[input_off..input_len], &mut output_buf);
            //            eprintln!("input len: {} output len: {}", input_consumed, output_len * 8);
            input_off += input_consumed;
            output.write_all(as_raw_u8_slice(&output_buf[0..output_len]))?;
        }
    }

    eprintln!("T: encoding:      {:?}", start.elapsed());

    Ok(())
}

#[allow(clippy::needless_lifetimes)]
fn as_raw_u8_slice<'a>(words: &'a [u64]) -> &'a [u8] {
    unsafe { std::slice::from_raw_parts(words.as_ptr() as *const u8, words.len() * 8) }
}
