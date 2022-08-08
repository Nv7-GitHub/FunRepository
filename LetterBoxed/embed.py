with open("words.txt") as f:
  words = f.read().lower().split("\n")

out = open("src/tests/words.rs", "w+")

text = "#![allow(dead_code)]\n\npub fn test_data() -> String {\n\t\"" + ",".join(words) + "\".to_string()\n}\n"
out.write(text)
