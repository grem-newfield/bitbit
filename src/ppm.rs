use crate::uint;
use std::{
    fs::{read, File},
    io::{BufRead, Read, Write},
    path::Path,
};

struct RGB
{
    r: u8,
    g: u8,
    b: u8,
}
struct PPM
{
    height: uint,
    width:  uint,
    data:   Vec<u8>,
}
impl PPM
{
    pub fn new(
        height: uint,
        width: uint,
    ) -> PPM
    {
        let size = 3 * height * width;
        let buf = vec![0; size as usize];
        PPM {
            height,
            width,
            data: buf,
        }
    }
    fn buffer_size(&self) -> uint { 3 * self.height * self.width }
    fn get_offset(
        &self,
        x: uint,
        y: uint,
    ) -> Option<usize>
    {
        let offset = (y * self.width * 3) + (x + 3);
        if offset < self.buffer_size()
        {
            Some(offset as usize)
        }
        else
        {
            None
        }
    }
    pub fn get_pixel(
        &self,
        x: uint,
        y: uint,
    ) -> Option<RGB>
    {
        match self.get_offset(x, y)
        {
            Some(offset) =>
            {
                let r = self.data[offset];
                let g = self.data[offset + 1];
                let b = self.data[offset + 2];
                Some(RGB {
                    r,
                    g,
                    b,
                })
            }
            None => None,
        }
    }
    pub fn set_pixel(
        &mut self,
        x: uint,
        y: uint,
        color: RGB,
    ) -> bool
    {
        match self.get_offset(x, y)
        {
            Some(offset) =>
            {
                self.data[offset] = color.r;
                self.data[offset + 1] = color.g;
                self.data[offset + 2] = color.b;
                true
            }
            None => false,
        }
    }
    pub fn write_file(
        &self,
        filename: &str,
    ) -> std::io::Result<()>
    {
        let path = Path::new(filename);
        let mut file = File::create(&path)?;
        let header = format!("P6 {} {} 255\n", self.width, self.height);
        file.write(header.as_bytes())?;
        file.write(&self.data)?;
        Ok(())
    }
    pub fn read_file(
        &self,
        filename: &str,
    ) -> std::io::Result<()>
    {
        use std::io::{self, BufRead};
        let path = Path::new(filename);
        let mut file = File::open(&path)?;
        let mut data = Vec::new();
        let data_len = file.read_to_end(&mut data);
        let mut cursor = io::Cursor::new(data);

        // read name
        // let mut name = Vec::new();
        // let num_bytes =
        // cursor.read_until(b'\0', &mut name).expect("reading from cursor won't fail");
        // assert_eq!(num_bytes, 7);
        // assert_eq!(name, b"Ferris\0");

        // skip fun fact
        // let num_bytes = cursor.skip_until(b'\0').expect("reading from cursor won't fail");
        // assert_eq!(num_bytes, 30);

        // read animal type
        // let mut animal = Vec::new();
        // let num_bytes =
        // cursor.read_until(b'\0', &mut animal).expect("reading from cursor won't fail");

        let mut header = Vec::new();
        let num_bytes = cursor.read_until(b'\n', &mut header)?; // read header line
        println!("HEADER: {:?}", header);

        // let buf = BufRead::lines(file);
        // let header = format!("P6 {} {} 255\n", self.width, self.height);
        let data = read(path)?;
        // let width = &data[3..4];
        // let height= &data[3..4];
        // let header = String::new();
        // file.write(header.as_bytes())?;
        // file.write(&self.data)?;
        Ok(())
    }
}
