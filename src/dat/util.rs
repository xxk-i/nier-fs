/*
struct {
    char    id[4];
    uint32  fileNumber;
    uint32  fileOffsetsOffset <format=hex>;
    uint32  fileExtensionsOffset <format=hex>;
    uint32  fileNamesOffset <format=hex>;
    uint32  fileSizesOffset <format=hex>;
    uint32  hashMapOffset <format=hex>;
} header;
*/

use binrw::*;
use binrw::io::Seek;
use binrw::io::Read;

// Name table entries are padded to the length of the longest name
// The table is padded to a 4 byte alignment
fn parse_names<R: Read + Seek>(reader: &mut R, _ro: &ReadOptions, args: (u32, u32)) -> BinResult<Vec<NullString>>
{
    let file_count = args.0;
    let length = args.1;
    let mut result: Vec<NullString> = Vec::new();

    for _n in 0..file_count {
        let name = reader.read_be::<NullString>().unwrap();
        let len = name.len() as u32;
        result.push(name);

        reader.seek(io::SeekFrom::Current((length - len - 1) as i64))?;
    }

    Ok(result)
}

// Dumps rest of data until EOF into DAT.data
fn parse_data<R: Read + Seek>(reader: &mut R, _ro: &ReadOptions, _: ()) -> BinResult<Vec<u8>>
{
    let mut data: Vec<u8> = Vec::new();
    println!("bytes read: {}", reader.read_to_end(&mut data).unwrap());

    Ok(data)
}

#[binrw]
#[br(little)]
#[br(magic = b"DAT\0")]
pub struct Metadata {
    pub file_count: u32,
    pub offsets_offset: u32,
    pub extensions_offset: u32,
    pub name_table_offset: u32,
    pub sizes_offset: u32,
    pub hashmap_offset: u32,
    _padding: u32
}

// binrw meme
// #[binrw]
// #[br(little, import { count: u32, length: u32 } )]
// pub struct NameEntry {
//     #[br(align_after = self.name.length() - length)]
//     pub name: NullString
// }

#[binrw]
#[br(little, import(file_count: u32))]
pub struct NameTable {
    #[br(little)]
    pub longest_length: u32,

    #[br(align_after = 4)]
    #[br(args(file_count, longest_length), parse_with = parse_names)]
    pub names: Vec<NullString>

    // #[br(align_after = 4)]
    // #[br(args { count: file_count as usize, length: longest_length })]
    // pub names: Vec<NameEntry>
}

#[binrw]
#[br(little, import(file_count: u32))]
pub struct HashMap {
    #[br(little)]
    pub hash_shift: u32,    // Usually referred to as preHashShift
    pub sorted_indices_offset: u32, // Usually referred to as bucketOffsetsOffset
    pub hashes_offset: u32,
    pub file_indices_offset: u32,

    #[br(count = 1 << (31 - hash_shift))]
    pub buckets: Vec<u16>,

    #[br(count = file_count)]
    pub hashes: Vec<u32>,

    #[br(count = file_count)]
    pub file_indices: Vec<u16>
}

#[binrw]
#[br(little)]
pub struct DAT {
    pub metadata: Metadata,

    #[br(count = metadata.file_count)]
    pub offsets: Vec<u32>,

    #[br(count = metadata.file_count)]
    pub extensions: Vec<NullString>,

    #[br(args(metadata.file_count))]
    pub name_table: NameTable,

    #[br(count = metadata.file_count)]
    pub sizes: Vec<u32>,

    #[br(align_after = 16)]
    #[br(args(metadata.file_count))]
    pub hashmap: HashMap,

    #[br(parse_with = parse_data)]
    pub data: Vec<u8>
}
