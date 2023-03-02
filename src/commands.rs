use std::str::FromStr;

use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::Result;

pub fn encode(encode_args: EncodeArgs) -> Result<()> {
    let mut png = Png::from_file(encode_args.file_path.clone())?;
    let chunk_type = ChunkType::from_str(encode_args.chunk_type.as_str())?;
    let chunk = Chunk::new(chunk_type, Vec::from(encode_args.message.as_bytes()));
    png.append_chunk(chunk);
    png.write_file(encode_args.output_file.unwrap_or(encode_args.file_path))
}

pub fn decode(decode_args: DecodeArgs) -> Result<()> {
    let png = Png::from_file(decode_args.file_path)?;
    let chunk = png.chunk_by_type(decode_args.chunk_type.as_str());
    println!("{}", String::from_utf8(Vec::from(chunk.unwrap().data())).unwrap());
    Ok(())
}

pub fn remove(remove_args: RemoveArgs) -> Result<()> {
    let mut png = Png::from_file(remove_args.file_path.clone())?;
    png.remove_chunk(remove_args.chunk_type.as_str())?;
    png.write_file(remove_args.file_path)
}

pub fn print(print_args: PrintArgs) -> Result<()> {
    let png = Png::from_file(print_args.file_path)?;
    for chunk in png.chunks() {
        println!("{}", chunk);
    }
    Ok(())
}