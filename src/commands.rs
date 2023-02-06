use std::{fs, str::FromStr};

use crate::{
    args::{Cli, Commands},
    chunk::Chunk,
    chunk_type::ChunkType,
    get_crc, get_file_as_byte_vec,
    png::Png,
    print_png_data,
};

pub fn execute(cli: &Cli) {
    match &cli.command {
        Commands::Encode(x) => {
            let file = get_file_as_byte_vec(&x.location);
            let mut png = Png::try_from(&file[..]).unwrap();
            let new_chunk = Chunk {
                len: match x.message.as_ref() {
                    Some(x) => u32::try_from(x.len()).unwrap(),
                    None => panic!("No message provided"),
                },
                chunk_type: match x.op_code.as_ref() {
                    Some(x) => ChunkType::from_str(&x).unwrap(),
                    None => panic!("Op code doesn't match"),
                },
                chunk_data: match x.message.as_ref() {
                    Some(x) => x.as_bytes().to_vec(),
                    None => panic!("Invalid Message"),
                },
                crc: get_crc(
                    ChunkType::from_str(&x.op_code.as_ref().unwrap()).unwrap(),
                    x.message.as_ref().unwrap().as_bytes().to_vec(),
                ),
            };
            png.append_chunk(new_chunk);
            match x.output.as_ref() {
                Some(a) => {
                    fs::write(a, png.as_bytes()).expect("Unable to write file");
                    println!(
                        "Encoding to the file {:?} with the message {:?} with the following op_code {:?} and saving the file in in the location {:?}, New Png value is {:?}]",
                        x.location,
                        x.message.as_ref().unwrap(),
                        x.op_code.as_ref().unwrap(),
                        x.output.as_ref().unwrap(),
                        print_png_data(png)
                    )
                }
                None => {
                    fs::write(&x.location, png.as_bytes()).expect("Unable to write file");
                    println!(
                        "Encoding to the file {:?} with the message {:?} with the following op_code {:?} and saving, New Png value is {:?}]",
                        x.location,
                        x.message.as_ref().unwrap(),
                        x.op_code.as_ref().unwrap(),
                        print_png_data(png)
                    )
                }
            }
        }
        Commands::Decode(x) => {
            let file = get_file_as_byte_vec(&x.location);
            let png = Png::try_from(&file[..]).unwrap();
            let output = png.chunk_by_type(x.op_code.as_ref().unwrap().as_str());

            match output {
                Some(x) => println!("Message is {:?}", std::str::from_utf8(x.data()).unwrap()),
                None => println!("Can't find the goven op_code in the png file!"),
            }
        }
        Commands::Remove(x) => {
            let file = get_file_as_byte_vec(&x.location);
            let mut png = Png::try_from(&file[..]).unwrap();
            let output = png.chunk_by_type(x.op_code.as_ref().unwrap().as_str());

            match output {
                Some(a) => {
                    let chunk_str = a.chunk_type.to_string();
                    png.remove_chunk(&chunk_str)
                        .expect("Unable to remove chunk from PNG!");
                    fs::write(&x.location, png.as_bytes()).expect("Unable to write file");
                    println!("Removed the Data associated with the Chunk : {}", chunk_str)
                }
                None => println!("Given ChunkType don't exist for this PNG"),
            }
        }
        Commands::Print(x) => {
            let file = get_file_as_byte_vec(&x.location);
            let png = Png::try_from(&file[..]).unwrap();
            print_png_data(png);
        }
    }
}
