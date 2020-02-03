

extern crate fat;

use std::env;
use std::str;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = match args.get(1) {
        Some(arg) => Path::new(arg),
        _ => Path::new("/home/zack/src/fat/test.img")
    };
    let mut buf = [0; 36];
    let mut file = File::open(path).expect("Failed to open file");
    file.read_exact(&mut buf);

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

    println!("---");

    let mut ebp_buf = [0; 476];
    file.read_exact(&mut ebp_buf);
    let ebp = fat::unpack_ebp(ebp_buf);
    println!("sectors per FAT: {}", ebp.sectors_per_fat);
    println!("fat_version: {}.{}",
             ebp.fat_version_major, ebp.fat_version_minor);
    println!("root cluster: {}", ebp.root_cluster);
    println!("fsinfo sector: {}", ebp.fsinfo_sector);
    println!("backup boot sector: {}", ebp.backup_boot_sector);
    println!("drive number: {}", ebp.drive_number);
    println!("volume id: {}", ebp.volume_id);
    println!("volume label: {}", str::from_utf8(&ebp.volume_label).unwrap());
    println!("system id: {}", str::from_utf8(&ebp.system_id).unwrap());
}

