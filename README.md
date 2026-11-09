# Smart Review

Powered by the SM-2 algorithm, efficiently apply the [spaced repetition](https://en.wikipedia.org/wiki/Spaced_repetition) technique by creating items, giving them a title and a description, and rating the quality of your reviews.

## Installation

### crates.io

For the sake of convenience and simplicity, it's highly recommended to install it via Rust's package manager by running:
```sh
cargo install smart-review
```

### From source

However, you can also download the source code and compile `smart-review` yourself.

1. Clone this repository to your local machine by using the command: 
```bash
git clone https://github.com/daniel-valencia-ts/smart-review.git
``` 

2. From the cloned repository, execute:
```bash
cargo run
```

## Usage

Run `smart-review` or `sr` with any of the following options:

- add a new item:
```bash
sr new 
```

- review an item, where `arg` must be either the item's ID or title:
```bash
sr review arg
```

- remove the first item that matches the given ID or title:
```bash
sr remove arg
```

- display a single item or all of them if no ID or title is given:
```bash
sr show arg # optional
```