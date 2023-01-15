mod util;

use std::fs::File;
use std::path::PathBuf;
use binrw::*;
use util::DAT;

pub fn print_debug(dat: DAT) {
    println!("file_count: {:04X}", dat.metadata.file_count);
    println!("offset_table_offset: {:04X}", dat.metadata.offsets_offset);
    println!("extension_table_offset {:04X}", dat.metadata.extensions_offset);
    println!("names_table_offset: {:04X}", dat.metadata.name_table_offset);
    println!("sizes_table_offset: {:04X}", dat.metadata.sizes_offset);
    println!("hashmap_offset: {:04X}", dat.metadata.hashmap_offset);

    println!("name length {:#02X}", dat.name_table.longest_length);
    println!("file offsets {:#08X?}", dat.offsets);
    println!("file sizes {:#08X?}", dat.sizes);

    println!("~~~~~~~~~~~~~~~");

    println!("{:04X}", dat.hashmap.hash_shift);
    println!("{:04X}", dat.hashmap.sorted_indices_offset);
    println!("{:04X}", dat.hashmap.hashes_offset);
    println!("{:04X}", dat.hashmap.file_indices_offset);

    println!("{:#08X?}", dat.hashmap.hashes);
    println!("{:#08X?}", dat.hashmap.file_indices);
    // for offset in dat.offsets {
    //     println!("{:04X}", offset);
    // }

    for name in dat.name_table.names {
        println!("{}", name)
    }

    println!("{}", String::from_utf8_lossy(&dat.data[0..4]));
}

pub fn unpack(mut f: File, o: &PathBuf) -> Result<(), &'static str> {
    let dat: DAT = f.read_le().unwrap();
    
    print_debug(dat);

    Ok(())
}