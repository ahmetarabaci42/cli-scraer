# 🕷️ CLI Scraper

A simple and async **command-line web scraper** built with Rust.  
It fetches a web page, extracts **titles (h1–h6)** and **links (a href)**, and outputs them in **JSON** or line-based format.

---

## ✨ Features
- 🔎 Extracts all headings (`<h1>` … `<h6>`)
- 🌐 Collects all links (`<a href="...">`)
- ⚡ Async & fast (powered by `tokio` + `reqwest`)
- 📦 Outputs in JSON (pretty) or simple lines
- 🛠️ Easy to extend (add depth, concurrency, filtering)

---

## 📦 Installation
git clone https://github.com/your-username/cli-scraper.git
cd cli-scraper
cargo build --release

The compiled binary will be at:
target/release/cli-scraper

---

## 🚀 Usage
cargo run -- <URL> --format <json|lines>

### Examples
cargo run -- https://www.rust-lang.org --format json

Output:
{
  "titles": ["Rust", "Why Rust?", "Performance", "..."],
  "links": ["/learn", "https://github.com/rust-lang", "..."]
}

cargo run -- https://www.rust-lang.org --format lines

Output:
[TITLE] Rust
[TITLE] Why Rust?
[LINK] https://github.com/rust-lang
...

---

## ⚙️ Dependencies
- tokio – async runtime
- reqwest – HTTP client
- clap – CLI argument parser
- scraper – HTML parsing
- serde, serde_json – structured output
- anyhow – error handling

---

## 🛠️ Roadmap
- [ ] Add concurrency for multiple URLs
- [ ] Add crawl depth option (--depth)
- [ ] Export CSV output
- [ ] Robots.txt compliance

---

## 🤝 Contributing
Pull requests and issues are welcome.  
Follow Conventional Commits style for commits.

---

