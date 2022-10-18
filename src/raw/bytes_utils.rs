use byteorder::{BigEndian, ReadBytesExt};

pub trait ByteUtils: ReadBytesExt {
    fn read_u1(&mut self) -> std::io::Result<u8>;
    fn read_u2(&mut self) -> std::io::Result<u16>;
    fn read_u4(&mut self) -> std::io::Result<u32>;

    /// Read array in the shape of length items...
    fn read_array<B, F>(&mut self, count: usize, f: F) -> std::io::Result<Vec<B>>
    where
        F: FnMut(&mut Self) -> std::io::Result<B>;
}

impl<Bytes> ByteUtils for Bytes
where
    Bytes: ReadBytesExt,
{
    fn read_u1(&mut self) -> std::io::Result<u8> {
        self.read_u8()
    }

    fn read_u2(&mut self) -> std::io::Result<u16> {
        self.read_u16::<BigEndian>()
    }

    fn read_u4(&mut self) -> std::io::Result<u32> {
        self.read_u32::<BigEndian>()
    }

    fn read_array<B, F>(&mut self, count: usize, mut f: F) -> std::io::Result<Vec<B>>
    where
        F: FnMut(&mut Self) -> std::io::Result<B>,
    {
        let mut result: Vec<B> = vec![];
        result.reserve(count);
        for _ in 0..count {
            result.push(f(self)?);
        }
        Ok(result)
    }
}
