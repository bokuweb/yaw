use std::io::{Error, Read};

pub fn read_next<R: Read>(reader: &mut R) -> Result<u8, Error> {
    let mut buf = [0u8; 1];
    reader.read_exact(&mut buf)?;
    Ok(buf[0])
}

pub fn read_bytes<R: Read>(reader: &mut R, len: usize) -> Result<Vec<u8>, Error> {
    let mut b = vec![0u8; len];
    reader.read_exact(&mut b)?;
    Ok(b)
}

pub fn read_u64<R: Read>(reader: &mut R) -> Result<u64, Error> {
    Ok(read_bytes(reader, 8)?
        .iter()
        .rev()
        .fold(0, |x, &i| x << 8 | u64::from(i)))
}

pub fn read_u32<R: Read>(reader: &mut R) -> Result<u32, Error> {
    Ok(read_bytes(reader, 4)?
        .iter()
        .rev()
        .fold(0, |x, &i| x << 8 | u32::from(i)))
}

#[test]
fn it_read_next() {
    let b = vec![0x08, 0x09];
    let v = read_next(&mut &b[..]).unwrap();
    assert_eq!(v, 0x08);
}

#[test]
fn it_read_bytes() {
    let b = vec![0x08, 0x09, 0x0a, 0x0b];
    let v = read_bytes(&mut &b[..], 2).unwrap();
    assert_eq!(v, vec![0x08, 0x09]);
}
