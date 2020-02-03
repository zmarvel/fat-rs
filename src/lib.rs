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
pub struct EBP {
    pub sectors_per_fat: u32,
    pub flags: u16,
    pub fat_version_major: u8,
    pub fat_version_minor: u8,
    pub root_cluster: u32,
    pub fsinfo_sector: u16,
    pub backup_boot_sector: u16,
    pub reserved: [u8; 12], // should be zero when formatted
    pub drive_number: u8, // 0x00 for floppy, 0x80 for hard disk
    pub nt_flags: u8,
    pub signature: u8, // 0x28 or 0x29
    pub volume_id: u32,
    pub volume_label: [u8; 11], // zero-padded string
    pub system_id: [u8; 8], // should be "FAT32   "
    pub boot_code: [u8; 420],
    pub bootable_part_signature: u16 // 0xaa55: indicates bootable
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

fn unpack_u32_le(raw: &[u8]) -> u32 {
    u32::from_le((raw[0] as u32) << 24 |
                 (raw[1] as u32) << 16 |
                 (raw[2] as u32) << 8 |
                 raw[3] as u32)
}

fn unpack_u16_le(raw: &[u8]) -> u16 {
    u16::from_le((raw[0] as u16) << 8 |
                 raw[1] as u16)
}

pub fn unpack_ebp(raw: [u8; 476]) -> EBP {
    let mut ebp = EBP {
        sectors_per_fat: unpack_u32_le(&raw[0..4]),
        flags: unpack_u16_le(&raw[4..6]),
        fat_version_major: raw[6],
        fat_version_minor: raw[7],
        root_cluster: unpack_u32_le(&raw[8..12]),
        fsinfo_sector: unpack_u16_le(&raw[12..14]),
        backup_boot_sector: unpack_u16_le(&raw[14..16]),
        reserved: [0; 12],
        drive_number: raw[28],
        nt_flags: raw[29],
        signature: raw[30],
        volume_id: unpack_u32_le(&raw[31..35]),
        volume_label: [0; 11], // 35..46
        system_id: [0; 8], // 46..54
        boot_code: [0; 420], // 54..474
        bootable_part_signature: unpack_u16_le(&raw[474..476])
    };

    ebp.volume_label.copy_from_slice(&raw[35..46]);
    ebp.system_id.copy_from_slice(&raw[46..54]);
    ebp.boot_code.copy_from_slice(&raw[54..474]);

    ebp
}

