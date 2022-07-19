use std::io::{BufReader, Read, Seek};

// Read x byte
pub fn read_x<R: Read>(reader: &mut BufReader<R>, size: usize) -> Result<Vec<u8>, std::io::Error> {
    let mut buf: Vec<u8> = Vec::new();
    buf.resize(size, 0);
    // Stack Overflow : https://stackoverflow.com/questions/30412521/how-to-read-a-specific-number-of-bytes-from-a-stream
    // なぜかVecが渡せる
    if let Err(err) = reader.read_exact(&mut buf) {
        Err(err)
    } else {
        Ok(buf)
    }
}

// Read 1 byte
pub fn read_8<R: Read>(reader: &mut BufReader<R>) -> Result<[u8; 1], std::io::Error> {
    let mut buf: [u8; 1] = [0; 1];

    if let Err(err) = reader.read_exact(&mut buf) {
        Err(err)
    } else {
        Ok(buf)
    }
}

// Read 4 byte
pub fn read_32<R: Read>(reader: &mut BufReader<R>) -> Result<[u8; 4], std::io::Error> {
    let mut buf: [u8; 4] = [0; 4];

    if let Err(err) = reader.read_exact(&mut buf) {
        Err(err)
    } else {
        Ok(buf)
    }
}

// Peep 1 byte (not move cursor)
pub fn peep_8<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<[u8; 1], std::io::Error> {
    let mut buf: [u8; 1] = [0; 1];

    if let Err(err) = reader.read_exact(&mut buf) {
        return Err(err);
    };

    if let Err(err) = reader.seek_relative(-1) {
        return Err(err);
    };

    Ok(buf)
}

pub fn read_unsigned_leb128<R: Read>(
    reader: &mut BufReader<R>,
    buffer: &mut u64,
) -> Result<u8, leb128::read::Error> {
    match leb128::read::unsigned(reader) {
        Ok(v) => *buffer = v,
        Err(err) => return Err(err),
    };

    Ok(usage_bytes_leb128_u(*buffer))
}

pub fn read_signed_leb128<R: Read>(
    reader: &mut BufReader<R>,
    buffer: &mut i64,
) -> Result<u8, leb128::read::Error> {
    match leb128::read::signed(reader) {
        Ok(v) => *buffer = v,
        Err(err) => return Err(err),
    };

    Ok(usage_bytes_leb128_s(*buffer))
}

pub fn usage_bytes_leb128_u(value: u64) -> u8 {
    let mut ord: u8 = 1;
    let base: i64 = 2;
    while (value as i64) >= base.pow(7 * (ord as u32)) {
        ord += 1;
    }

    ord
}

pub fn usage_bytes_leb128_s(value: i64) -> u8 {
    let mut ord: u8 = 1;
    let base: i64 = 2;

    // unsigned intにおいて、例えば、-xのビット数とx-1のビット数は同じ (where x > 0)
    let value = if value >= 0 { value } else { -(value + 1) };

    let sign_bit: u32 = 1;
    while value >= base.pow(7 * (ord as u32) - sign_bit) {
        ord += 1;
    }

    ord
}
