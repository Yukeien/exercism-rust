#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut vector: Vec<u8> = Vec::new();

    for value in values {
        let mut vec_number: Vec<u8> = Vec::new();
        let mut number: u64 = value.clone() as u64;
        let mut first: bool = true;

        if number == 0 {
            vector.push(0);
        } else {
            while number != 0 {
                println!("{}", number);
                number = number << 1;
                let mut byte: u8 = ((number as u8) >> 1) & 127;
                number = number >> 8;

                if !first {
                    byte = byte + 128;
                } else {
                    first = false;
                }

                vec_number.insert(0, byte);
            }
        }

        vector.append(&mut vec_number);
    }

    vector
}

pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut vector: Vec<u32> = Vec::new();
    let mut result: u64 = 0;
    let mut index: u32 = 0;

    while (index as usize) < bytes.len() {
        while (index as usize) < bytes.len() {
            let byte = bytes[index as usize];

            if byte & 128 == 128 && ((index + 1) as usize) == bytes.len() {
                return Result::Err(Error::IncompleteNumber)
            }

            println!("{:#032b}", result);
            result = (result << 7) | (byte & 127) as u64;

            if result > (u32::MAX) as u64 {
                return Result::Err(Error::Overflow)
            }

            if byte & 128 != 128 {
                break;
            }

            index += 1;
        }

        vector.push(result as u32);
        result = 0;

        index += 1;
    }

    Result::Ok(vector)
}
