use std::fs::File;
use std::path::PathBuf;

pub fn unpack(f: File, o: &PathBuf) -> Result<(), &'static str> {
    println!("{:#08x}", f.metadata().unwrap().len());
    println!("{:#?}", o);
    Ok(())
}