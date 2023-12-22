use crc32fast::Hasher as Crc32Hasher;
use std::{
    io,
    io::Read,
    path::{Path, PathBuf},
};

pub fn hash(mut reader: impl Read, buffer: &mut [u8]) -> Result<u32, io::Error> {
    let mut hasher = Crc32Hasher::new();

    while let Ok(bytes_read) = reader.read(buffer) {
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(hasher.finalize())
}

pub fn hash_file_contents(path: impl AsRef<Path>, buffer: &mut [u8]) -> Result<u32, io::Error> {
    let file = std::fs::File::open(path.as_ref())?;
    hash(file, buffer)
}

pub fn walk_dir(dir: &Path, on_file_found: &mut dyn FnMut(&PathBuf)) -> io::Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }

    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            walk_dir(&path, on_file_found)?;
        } else {
            on_file_found(&path);
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    const TEST_TEXT: &[u8] = b"hello world";
    const TEST_TEXT_HASH: u32 = 0xD4A1185;

    #[test]
    fn test_hash() {
        let mut buffer = [0; 1024];
        {
            let mut cursor = Cursor::new(TEST_TEXT.to_vec());
            let hash = hash(&mut cursor, &mut buffer).unwrap();
            assert_eq!(hash, TEST_TEXT_HASH);
        }
        {
            let mut cursor = Cursor::new(b"world, hello ".to_vec());
            let hash = hash(&mut cursor, &mut buffer).unwrap();
            assert_ne!(hash, TEST_TEXT_HASH);
        }
    }

    #[test]
    fn test_hash_file_contents() {
        let mut buffer = [0; 1024];

        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("hashed.txt");
        std::fs::write(&file, TEST_TEXT).unwrap();

        let hash = hash_file_contents(file.to_str().unwrap(), &mut buffer).unwrap();
        assert_eq!(hash, TEST_TEXT_HASH);

        std::fs::remove_file(file).unwrap();
    }
}
