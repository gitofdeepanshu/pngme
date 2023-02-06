use std::fs::{self, File};
use std::io::Read;
use std::str::FromStr;

mod args;
mod chunk;
pub mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

use args::{Cli, Commands};
use chunk::{calculate_crc, Chunk};
use clap::Parser;
use commands::execute;
use png::Png;

use crate::chunk_type::ChunkType;

fn main() -> Result<()> {
    let cli = Cli::parse();

    execute(&cli);
    Ok(())
}

fn get_crc(chunk_type: ChunkType, mut chunk_data: Vec<u8>) -> u32 {
    let mut v = chunk_type.bytes().to_vec();
    v.append(&mut chunk_data);
    calculate_crc(&v)
}
fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");

    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}
fn print_png_data(png: Png) {
    let printable_chunks: Vec<String> = png
        .chunks()
        .iter()
        .map(|x| {
            format!(
                "[Len:{} ChunkType:{} Data:{:?} crc:{}]",
                x.length(),
                x.chunk_type,
                (x.data().len()),
                x.crc()
            )
        })
        .collect();
    println!("{:#?}", printable_chunks);
}
