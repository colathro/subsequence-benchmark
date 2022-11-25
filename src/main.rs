use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    time,
};

/// This shows up about 40-45% faster than find_subsequence.
#[inline]
fn find_subsequence_v2(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    let len = needle.len();

    let mut index = 0;
    for window in haystack.windows(len) {
        for i in 0..len {
            if needle[i] != window[i] {
                break;
            }
            if i == len - 1 {
                return Some(index);
            }
        }
        index += 1;
    }

    return None;
}

#[inline]
fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

fn main() -> io::Result<()> {
    let start = time::Instant::now();

    const CAP: usize = 65536;
    let file = File::open("./file.bin")?;
    let mut reader = BufReader::with_capacity(CAP, file);

    let needle: &[u8] = &[45, 45, 98, 111, 117, 110, 100, 97, 114, 121, 45, 45];

    let mut ind: usize = 0;
    let mut count: i32 = 0;

    loop {
        let length = {
            let buffer = reader.fill_buf()?;
            // do stuff with buffer here
            match find_subsequence_v2(&buffer, &needle) {
                Some(index) => {
                    count += 1;
                    ind += index;
                    println!("Number of chunks {:?}", count);
                    println!("Found index {:?}.", ind);
                }
                None => {
                    count += 1;
                    ind += buffer.len();
                }
            }
            buffer.len()
        };
        if length == 0 {
            break;
        }
        reader.consume(length);
    }

    println!("Elapsed Time: {}", start.elapsed().as_millis());

    Ok(())
}
