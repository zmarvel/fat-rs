

extern crate fat;

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::error::Error;

fn main() {
    let path = Path::new("/home/zack/src/fat/test.img");
    let mut buf = [0; 36];
    let mut file = match File::open(&path) { 
        Err(err) => panic!("Failed to open {}: {}",
                           path.display(), err.description()),
        Ok(mut f) => f.read_exact(&mut buf)
    };

    let bpb = fat::unpack_bpb(buf);
    println!("{:?}", bpb);

    println!("jmp: {:x} {:x} {:x}", bpb.jmp[0], bpb.jmp[1], bpb.jmp[2]);
    match std::str::from_utf8(&bpb.oem_id) {
        Ok(s) => println!("oem_id: {}", s),
        Err(_) => ()
    };
    println!("bytes per sector: {}", bpb.bytes_per_sector);
    println!("reserved sectors: {}", bpb.reserved_sectors);
    println!("directory entries: {}", bpb.dir_entries);
    println!("logical volume sectors: {}", bpb.lv_sectors);
    println!("sectors per FAT: {}", bpb.sectors_per_fat);
    println!("heads: {}", bpb.heads);
    println!("hidden sectors: {}", bpb.hidden_sectors);
    println!("large logical volume sectors: {}", bpb.large_lv_sectors);
}

