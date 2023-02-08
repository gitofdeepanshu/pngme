# PNGME Message Encoder/Decoder in Rust
Introducing a program that enables you to encode messages into PNG files, decode messages stored in PNG files, remove messages from PNG files, and print a list of PNG chunks that can be searched for messages. With this program, you can securely store and transmit important information within a PNG image file. Whether you need to hide a message from prying eyes or store data in a compact format, this program provides an efficient and reliable solution.

## RESOURCES
1. PNG SPECS: http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
2. Check-sum : https://docs.rs/crc/2.1.0/crc/struct.Crc.html#method.checksum-1

## Features
1. Encode a message into a PNG file
2. Decode a message stored in a PNG file
3. Remove a message from a PNG file
4. Print a list of PNG chunks that can be searched for messages

## Prerequisites
- A basic understanding of cryptography and its principles
- Familiarity with Rust programming language

## Running the code
1. Clone the repository using `git clone https://github.com/gitofdeepanshu/pngme.git`
2. Change to the project directory using `cd pngme`
3. Run the code using `cargo run`


## Sample Usage
`pngme encode ./dice.png ruSt "This is a secret message!`

`pngme decode ./dice.png ruSt`

`pngme remove ./dice.png ruSt`

`pngme print ./dice.png`

## Contributing
If you have any suggestions or improvements to the code, feel free to create a pull request.

## License
This project is licensed under the [MIT License](LICENSE).


