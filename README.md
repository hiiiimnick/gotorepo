# gotorepo

Small utility to open the repository from the current directory in the browser

## Installation

### Prerequisites:
Rust version >= 1.95.0

### Installation steps

Clone Repository, enter the directory and run 
```
cargo install --path .
```
or in one command: 

```
git clone https://github.com/hiiiimnick/gotorepo.git && cd gotorepo && cargo install --path .
```

This will install the Binary into your Rust binary directory, usually in ```~/.cargo/bin```
Ensure this path is in your PATH to make it available as command


## Usage

run ```gotorepo``` in the directory of a git repository. 
