use std::io::{BufReader, Read, Seek};

// Read x byte
pub fn read_x<R: Read>(reader: &mut BufReader<R>, size: usize) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    buf.resize(size, 0);
    // Stack Overflow : https://stackoverflow.com/questions/30412521/how-to-read-a-specific-number-of-bytes-from-a-stream
    // なぜかVecが渡せる
    let rs = reader.read(&mut buf).unwrap();
    if rs != size {
        panic!(" > Error: Not enough raw bytes");
    }

    buf
}

// Read 1 byte
pub fn read_8<R: Read>(reader: &mut BufReader<R>) -> [u8; 1] {
    let mut buf: [u8; 1] = [0; 1];

    let rs = reader.read(&mut buf).unwrap();
    if rs != 1 {
        panic!(" > Error: Not enough raw bytes");
    }

    buf
}

// Read 4 byte
pub fn read_32<R: Read>(reader: &mut BufReader<R>) -> [u8; 4] {
    let mut buf: [u8; 4] = [0; 4];

    let rs = reader.read(&mut buf).unwrap();
    if rs != 4 {
        panic!(" > Error: Not enough raw bytes");
    }

    buf
}

// Peep 1 byte (not move cursor)
pub fn peep_8<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<[u8; 1], String> {
    let mut buf: [u8; 1] = [0; 1];

    let rs = reader.read(&mut buf).unwrap();
    if rs != 1 {
        // panic!(" > Error: Not enough raw bytes");
        // println!(" > Error: Not enough raw bytes");
        return Err(String::from("Not enough raw bytes"));
    }

    match reader.seek_relative(-1) {
        Err(err) => panic!(" > Error: {:?}", err),
        Ok(_) => (),
    };

    Ok(buf)
}
