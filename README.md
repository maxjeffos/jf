# jf - a super simple JSON formatter

A simple command line tool built in Rust that pretty prints JSON strings. It takes an unformatted single-line JSON from the command line arguments and pretty formats it. If the JSON is invalid, it will print an error message along with the location of the parsing error.

## Installation

Make sure you have Rust and Cargo installed on your machine. If not, please follow the [official guide](https://www.rust-lang.org/tools/install) to install Rust and Cargo.

```
git clone https://github.com/maxjeffos/jf.git
cd jf
```

## Usage

```
cargo run 'your_json_string'
```

For exampe:

```
cargo run '{"name":"John G","age":42,"city":"New York"}'
```

This will print:

```
{
  "age": 42,
  "city": "New York",
  "name": "John G"
}
```

If the JSON is invalid, the tool will print an error message along with the location of the error. For example:

```
cargo run '{"name"":"John G","age":42,"city":"New York"}'
```

This will print:

```
Error parsing JSON: expected `:` at line 1 column 8
{"name"":"John G","age":42,"city":"New York"}
```

## License

This project is licensed under the WTFPL license.
