#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}


/* Everything is little-endian. Multi-byte elements of this struct are
 * stored as arrays so values can be interpreted after the endianness of
 * the host system has been determined. */

/* NOTE: unaligned references to a struct with repr(packed) -- which is what
 * this code will require -- are unsafe. So rather than referencing members
 * of this struct, we are limited to reading/writing values.
 */

#[derive(Debug)]
pub struct BPB {
    pub jmp: [u8; 3],
    pub oem_id: [u8; 8],
    pub bytes_per_sector: u16,
    pub sectors_per_cluster: u8,
    pub reserved_sectors: u16, // boot record sectors included
    pub fats: u8,
    pub dir_entries: u16,
    // sectors in logical volume. if the number exceeds the capacity of two
    // bytes, this is stored in the 4 bytes at the end of this struct
    pub lv_sectors: u16,
    pub media_descriptor_type: u8,
    pub sectors_per_fat: u16, // fat32 uses 4 bytes in EBR instead
    pub sectors_per_track: u16,
    pub heads: u16,
    pub hidden_sectors: u32,
    pub large_lv_sectors: u32
}
// total size: 36 bytes

/* For FAT32 -- differs for FAT12 and FAT16 */
struct EBP {
    sectors_per_fat: [u8; 4],
    flags: [u8; 2],
    fat_version: [u8; 2],
    root_cluster: [u8; 4],
    fsinfo_sector: [u8; 2],
    backup_boot_sector: [u8; 2],
    reserved: [u8; 12], // should be zero when formatted
    drive_number: u8, // 0x00 for floppy, 0x80 for hard disk
    nt_flags: u8,
    signature: u8, // 0x28 or 0x29
    volume_id: [u8; 4],
    volume_label: [u8; 11], // zero-padded string
    system_id: [u8; 8], // should be "FAT32   "
    boot_code: [u8; 420],
    bootable_part_signature: [u8; 2] // 0xaa55: indicates bootable
}
// total size: 476 bytes

struct Directory {
    filename: [u8; 11],
    attrs: u8,
    nt_reserved: u8,
    creation_time_ds: u8, // tenths of a second
    creation_time: [u8; 2], // 5/6/5: hr/min/s
    creation_date: [u8; 2], // 7/4/5: y/m/d
    last_accessed: [u8; 2], // date; same format as creation_date
    cluster_number_h: [u8; 2], // high 16 bits of entry's first cluster number
    last_mod_time: [u8; 2],
    last_mod_date: [u8; 2],
    cluster_number_l: [u8; 2], // low 16 bits of entry's first cluster number
    file_size: [u8; 4]
}
// total size: 32 bytes

/* Unpack BIOS parameter block */
pub fn unpack_bpb(raw: [u8; 36]) -> BPB {
    let mut bpb = BPB {
        jmp: [0; 3],
        oem_id: [0; 8],
        bytes_per_sector: u16::from_le((raw[11] as u16) << 8 | raw[12] as u16),
        sectors_per_cluster: raw[13],
        reserved_sectors: u16::from_le((raw[14] as u16) << 8 | raw[15] as u16),
        fats: raw[16],
        dir_entries: u16::from_le((raw[17] as u16) << 8 | raw[18] as u16),
        lv_sectors: u16::from_le((raw[19] as u16) << 8 | raw[20] as u16),
        media_descriptor_type: raw[21],
        sectors_per_fat: u16::from_le((raw[22] as u16) << 8 | raw[23] as u16),
        sectors_per_track: u16::from_le((raw[24] as u16) << 8 | raw[25] as u16),
        heads: u16::from_le((raw[26] as u16) << 8 | raw[27] as u16),
        hidden_sectors: u32::from_le((raw[28] as u32) << 24 |
                                     (raw[29] as u32) << 16 |
                                     (raw[30] as u32) << 8 |
                                     raw[31] as u32),
        large_lv_sectors: u32::from_le((raw[32] as u32) << 24 |
                                       (raw[33] as u32) << 16 |
                                       (raw[34] as u32) << 8 |
                                       raw[35] as u32)
    };

    bpb.jmp.copy_from_slice(&raw[0..3]);
    bpb.oem_id.copy_from_slice(&raw[3..11]);

    bpb
}
