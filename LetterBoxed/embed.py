with open("words.txt") as f:
  words = f.read().split("\n")

out = open("src/words.rs", "w+")

text = "#![allow(dead_code)]\n\npub const WORDS: &'static [&str] = &["

for word in words:
  text += f"\"{word.lower()}\", "

text += "];\n"
out.write(text)
