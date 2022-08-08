use std::{collections::VecDeque, fs::File, io::{BufReader, Read, Write}};

struct Encoder {
    prev: u8,
    inp: VecDeque<u8>,
    out: VecDeque<u8>,
}

const OFF: i16 = 16;
const MIN: u8 = 1;

impl Encoder {
    fn new(mut inp: VecDeque<u8>) -> Self {
        let prev = inp[0];
        let out = VecDeque::from(vec![inp[0]]);
        inp.pop_front();
        Self { prev, inp, out }
    }
    
    fn preprocess(val: u8) -> u8 {
        match val {
            32 => 123,
            123 => 32,
            _ => val
        }
    }

    fn encode(&mut self) {
        while self.inp.len() > 0 {
            let encodable = self.cnt_encodable(0);
            if encodable > MIN {
                // Write header
                self.out.push_back((1 << 7) | encodable);

                // Write bytes
                let mut b = 0;
                let mut remaining = 8;

                for _ in 0..encodable {
                    let val = Encoder::preprocess(self.inp.pop_front().unwrap());
                    let diff = (val as i16 - self.prev as i16 + OFF) as u8;
                    
                    let shiftcnt: i8 = 5 - remaining;
                    if shiftcnt < 0 { // Space left in byte
                        b = b | (diff << ((-shiftcnt) as u8)); // Add to b
                        remaining = -shiftcnt;
                    } else if shiftcnt == 0 { // Fills up byte
                        b = b | diff;
                        self.out.push_back(b);
                        b = 0;
                        remaining = 8;
                    } else if shiftcnt > 0 { // Fills up byte and goes into next
                        // Fill up & push
                        b = b | (diff >> (shiftcnt as u8));
                        self.out.push_back(b);

                        // Go to next
                        let leftcnt = 8 - (shiftcnt as u8);
                        b = diff << leftcnt; // Shift to start
                        remaining = leftcnt as i8;
                    }

                    self.prev = val;
                }

                // Flush
                if remaining != 8 {
                    self.out.push_back(b);
                }

                continue;
            }

            // Calc number of bytes not encodable
            let mut max = self.inp.len(); // Length segment can only be 7 bits
            if max >= 1 << 7 {
                max = (1 << 7) - 1;
            }
            let mut cnt = 1;
            for i in (cnt as usize)..max {
                if self.cnt_encodable(i) > MIN {
                    break;
                }
                cnt += 1;
            }

            // Write header (first byte is already 0)
            self.out.push_back(cnt);

            // Write bytes
            for _ in 0..cnt {
                let val = self.inp.pop_front().unwrap();
                self.out.push_back(val);
                self.prev = val;
            }
        }
    }

    fn cnt_encodable(&self, pos: usize) -> u8 {
        let mut cnt = 0;
        let mut max = self.inp.len(); // Length segment can only be 7 bits
        if max >= 1 << 7 {
            max = (1 << 7) - 1;
        }
        let mut prev = self.prev;
        if pos > 0 {
            prev = self.inp[pos - 1];
        }
        for i in pos..max {
            let diff = Encoder::preprocess(self.inp[i]) as i16 - prev as i16 + OFF;
            if diff < 0 || diff >= (1 << 5) {
                break;
            }

            cnt += 1;
            prev = self.inp[i];
        }
        cnt
    }
}


fn main() {
    // Read
    let f = File::open("inp.txt").unwrap();
    let mut reader = BufReader::new(f);
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).unwrap();
    
    // Make encoder
    let mut enc = Encoder::new(VecDeque::from(buf));
    enc.encode();

    // Save
    let mut out = File::create("out.txt").unwrap();
    out.write_all(&enc.out.make_contiguous()).unwrap();
}
