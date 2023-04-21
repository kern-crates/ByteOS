use vfscore::{INodeInterface, Stat, VfsResult};

pub struct Null;

impl INodeInterface for Null {
    fn read(&self, buffer: &mut [u8]) -> VfsResult<usize> {
        buffer.fill(0);
        Ok(buffer.len())
    }

    fn write(&self, buffer: &[u8]) -> VfsResult<usize> {
        Ok(buffer.len())
    }

    fn stat(&self, stat: &mut Stat) -> VfsResult<()> {
        stat.dev = 0;
        stat.ino = 1; // TODO: convert path to number(ino)
        stat.mode = 0o20000; // TODO: add access mode
        stat.nlink = 1;
        stat.uid = 1000;
        stat.gid = 1000;
        stat.size = 0;
        stat.blksize = 512;
        stat.blocks = 0;
        stat.rdev = 0; // TODO: add device id
        Ok(())
    }
}