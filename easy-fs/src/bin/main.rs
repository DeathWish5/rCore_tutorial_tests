extern crate easy_fs;
extern crate alloc;

use easy_fs::{
    BlockDevice,
    EasyFileSystem,
};
use std::fs::{File, OpenOptions, read_dir};
use std::io::{Read, Write, Seek, SeekFrom};
use std::sync::Mutex;
use alloc::sync::Arc;

const BLOCK_SZ: usize = 512;

struct BlockFile(Mutex<File>);

impl BlockDevice for BlockFile {
    fn read_block(&self, block_id: usize, buf: &mut [u8]) {
        let mut file = self.0.lock().unwrap();
        file.seek(SeekFrom::Start((block_id * BLOCK_SZ) as u64))
            .expect("Error when seeking!");
        assert_eq!(file.read(buf).unwrap(), BLOCK_SZ, "Not a complete block!");
    }

    fn write_block(&self, block_id: usize, buf: &[u8]) {
        let mut file = self.0.lock().unwrap();
        file.seek(SeekFrom::Start((block_id * BLOCK_SZ) as u64))
            .expect("Error when seeking!");
        assert_eq!(file.write(buf).unwrap(), BLOCK_SZ, "Not a complete block!");
    }
}

fn main() {
    easy_fs_pack("binfs.img", BIN_SRC, ".bin").expect("Error when packing binary files!");
    easy_fs_pack("elffs.img", ELF_SRC, ".elf").expect("Error when packing binary files!");
}

static TARGET_PATH: &str = "../user/build/";
static BIN_SRC: &str = "../user/build/bin/";
static ELF_SRC: &str = "../user/build/elf/";

fn easy_fs_pack(target: &str, source: &str, suffix: &str) -> std::io::Result<()> {
    let block_file = Arc::new(BlockFile(Mutex::new({
        let f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(format!("{}{}", TARGET_PATH, target))?;
        f.set_len(8192 * 512).unwrap();
        f
    })));
    // 4MiB, at most 4095 files
    let efs = EasyFileSystem::create(
        block_file.clone(),
        8192,
        1,
    );
    let root_inode = Arc::new(EasyFileSystem::root_inode(&efs));
    let apps: Vec<_> = read_dir(source)
        .unwrap()
        .into_iter()
        .map(|dir_entry| {
            let mut name_with_ext = dir_entry.unwrap().file_name().into_string().unwrap();
            name_with_ext.drain(name_with_ext.find('.').unwrap()..name_with_ext.len());
            name_with_ext
        })
        .collect();
    for app in apps {
        // load app data from host file system
        let mut host_file = File::open(format!("{}{}{}", source, app, suffix)).unwrap();
        let mut all_data: Vec<u8> = Vec::new();
        host_file.read_to_end(&mut all_data).unwrap();
        // create a file in easy-fs
        let inode = root_inode.create(app.as_str()).unwrap();
        // write data to easy-fs
        inode.write_at(0, all_data.as_slice());
    }
    // list apps
    for app in root_inode.ls() {
        println!("{}", app);
    }
    Ok(())
}