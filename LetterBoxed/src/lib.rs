fn is_valid(word: &String, allowed: &[[u8; 3]; 4]) -> bool {
    let mut side_num = -1;
    for c in word.bytes() {
        // Get side num
        let mut num = -1;
        for (i, side) in allowed.into_iter().enumerate() {
            for char in side {
                if *char == c {
                    num = i as i32;
                }
            }
        }

        // Check
        if num == -1 {
            return false // Invalid letters
        }
        if num == side_num {
            return false // Same side
        }
        side_num = num;
    }
    true
}

fn iddfs(v: &mut Vec<usize>, max_len: usize, allowed: &[[u8; 3]; 4], words: &Vec<String>, letter_map: &[Vec<usize>; u8::MAX as usize]) -> Option<Vec<usize>> {
    if v.len() == max_len { // Too long for this depth
        return None;
    }

    // Check if this guess is the solution
    let mut needed = [false; u8::MAX as usize];
    for side in allowed { // Set all needed chars to true
        for c in side {
            needed[*c as usize] = true;
        }
    }
    for ind in v.iter() { // Set all chars we have to false
        for c in words[*ind].as_bytes() {
            needed[*c as usize] = false;
        }
    }
    
    // Check if any spots are still true
    let mut done = true;
    for v in needed {
        if v {
            done = false;
            break;
        }
    }
    if done { // All spots are false, this is the solution
        return Some(v.clone());
    }

    // Go through all possible next words
    let last_word = &words[v[v.len() - 1]];
    let last_char = last_word.as_bytes()[last_word.len() - 1];
    for next in letter_map[last_char as usize].iter() {
        // Add word to guess & check
        v.push(*next);
        let res = iddfs(v, max_len, allowed, words, letter_map);
        if let Some(_) = res {
            return res;
        }
        
        // Didn't work, remove from end
        v.remove(v.len()-1);
    }

    // Nothing worked
    None
}

pub fn solve(allowed: [[u8; 3]; 4], wordlist: &Vec<String>) -> Vec<String> {
    // Filter words
    let mut words = Vec::new();
    for word in wordlist.iter() {
        if is_valid(word, &allowed) {
            words.push(word.to_string());
        }
    }

    // Build lettermap (map of first letter of word to all words with that letter)
    const VAL: Vec<usize> = Vec::new();
    let mut letter_map = [VAL; u8::MAX as usize];
    for (i, word) in words.iter().enumerate() {
        letter_map[word.as_bytes()[0] as usize].push(i);
    }

    // Solve using iterative deepening depth-first search
    let mut max = 0;
    loop {
        max += 1;
        for i in 0..words.len() {
            let res = iddfs(&mut vec![i], max, &allowed, &words, &letter_map);
            if let Some(v) = res { // Found solution!
                let mut out = Vec::with_capacity(v.len());
                for ind in v {
                    out.push(words[ind].clone());
                }
                return out;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod words;
    use words::*;

    #[test]
    fn test() {
        let allowed: [[u8; 3]; 4] = [[b'x', b'i', b'e'], [b'm', b'c', b'n'], [b'o', b'u', b'y'], [b'l', b'q', b'r']];
        let data = test_data();
        let words: Vec<String> = data.split(",").map(|v| {v.to_string()}).collect();
        let res = solve(allowed, &words);
        assert_eq!(res, vec!["oxymoronic".to_string(), "clinique".to_string()])
    }
}