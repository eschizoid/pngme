use std::convert::TryFrom;
use std::fs;

use crate::args::{ExtractArgs, InsertArgs, ListArgs, RemoveArgs};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;

pub fn extract(args: &ExtractArgs) -> Result<(), Box<dyn std::error::Error>> {
    let png = Png::try_from(fs::read(&args.input)?.as_ref())?;
    let chunk = png
        .chunks()
        .iter()
        .find(|chunk| chunk.chunk_type() == &args.chunk_type)
        .ok_or("Chunk not found")?;
    fs::write(&args.output, chunk.data())?;
    Ok(())
}

pub fn insert(args: &InsertArgs) -> Result<(), Box<dyn std::error::Error>> {
    let mut png = Png::try_from(fs::read(&args.input)?.as_ref())?;
    let chunk = Chunk::try_from(fs::read(&args.chunk)?.as_ref())?;
    png.insert_chunk(chunk)?;
    fs::write(&args.output, png.as_bytes())?;
    Ok(())
}

pub fn list(args: &ListArgs) -> Result<(), Box<dyn std::error::Error>> {
    let png = Png::try_from(fs::read(&args.input)?.as_ref())?;
    for chunk in png.chunks() {
        println!("{}", chunk.chunk_type());
    }
    Ok(())
}