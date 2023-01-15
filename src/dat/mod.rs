mod util;

use std::fs;
use std::io::{SeekFrom, Read, Write};
use std::{fs::File, io::Seek};
use std::path::PathBuf;
use binrw::*;
use util::DAT;

use prettytable::{Table, Row, Cell, row};

pub fn print_debug(dat: &DAT) {
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

    for name in &dat.name_table.names {
        println!("{}", name)
    }
}

pub fn print_files(dat: &DAT) {
    let mut table = Table::new();

    let names: Vec<String> = dat.name_table.names.iter().map(|name| name.to_string()).collect();
    let extensions: Vec<String> = dat.extensions.iter().map(|extension| extension.to_string()).collect();
    let offsets: Vec<String> = dat.offsets.iter().map(|offset| format!("0x{:08X}", offset)).collect();
    let sizes: Vec<String> = dat.sizes.iter().map(|size| format!("0x{:08X}", size)).collect();

    table.add_row(row!["Name", "Extension", "Offset", "Size"]);

    for n in 0..names.len() {
        table.add_row(Row::new(vec![
            Cell::new(&names[n]),
            Cell::new(&extensions[n]),
            Cell::new(&offsets[n]),
            Cell::new(&sizes[n])]));
    }

    table.printstd();
}

pub fn unpack(mut f: File, o: &PathBuf, verbose: bool) -> Result<(), &'static str> {
    let header: &DAT = &f.read_le().unwrap();

    fs::create_dir_all(o).unwrap_or_else(|error|
        panic!("Problem creating output directory! {}", error)
    );

    for n in 0..header.metadata.file_count {
        // Collect info
        let offset = header.offsets[n as usize];
        let size = header.sizes[n as usize];
        let mut data = vec![0u8; size as usize];

        // Read file into buffer
        f.seek(SeekFrom::Start(offset as u64)).unwrap_or_else(|error|
            panic!("Problem seeking to offset! Invalid DAT? {}", error)
        );
        f.read_exact(&mut data).unwrap_or_else(|error|
            panic!("Problem reading data into buffer! {}", error)
        );

        let file_name = header.name_table.names[n as usize].to_string();

        if verbose { println!("Writing {} to {:?}...", file_name, o); }

        // Write buffer to new file
        let mut output_path= o.clone();
        output_path.push(file_name);
        let mut output = File::create(output_path).unwrap_or_else(|error| 
            panic!("Problem creating output file! {}", error)
        );
        output.write(&data).unwrap_or_else(|error|
            panic!("Problem writing buffer to output file! {}", error)
        );
    } 

    if verbose { print_files(header) }

    Ok(())
}