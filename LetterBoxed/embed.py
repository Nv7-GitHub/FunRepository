with open("words.txt") as f:
  words = f.read().split("\n")

out = open("src/tests/words.rs", "w+")

text = "#![allow(dead_code)]\n\npub fn test_words() -> Vec<&'static str> {\n\tvec!["

for i, word in enumerate(words):
  if i > 0:
    text += ", "
  text += f"\"{word.lower()}\""

text += "]\n}\n"
out.write(text)
