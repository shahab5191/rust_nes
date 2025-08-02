mod mapper;
use mapper::{Mapper, Mapper0};
use std::io::{Read, Seek, SeekFrom};

struct Header {
    pub prg_rom_size: u8,
    pub chr_rom_size: u8,
    pub mapper1: u8,
    pub mapper2: u8,
    pub prg_ram_size: u8,
    pub tv_system1: u8,
    pub tv_system2: u8,
}

#[derive(Debug, Clone, Copy)]
pub enum ScreenMirroring {
    Single,
    Vertical,
    Horizontal,
    FourScreen,
}

#[derive(Debug, Clone)]
pub struct Cartridge {
    pub mapper: Box<dyn Mapper>,
    pub mirroring: ScreenMirroring,
}

impl Cartridge {
    pub fn new() -> Self {
        Cartridge {
            mapper: Box::new(Mapper0::new(vec![], vec![], false)),
            mirroring: ScreenMirroring::Single,
        }
    }

    pub fn load_ines_rom(&mut self, file_path: &str) -> Result<(), std::io::Error> {
        // Load the INES ROM header and data
        // Open the file
        let mut file = std::fs::File::open(file_path)?;

        println!("File length: {}", file.metadata()?.len());

        let mut header_bytes = [0u8; 16];
        file.read_exact(&mut header_bytes)?;
        if &header_bytes[0..4] != b"NES\x1A" {
            panic!("Invalid NES ROM file");
        }

        let header = Header {
            prg_rom_size: header_bytes[4] as u8,
            chr_rom_size: header_bytes[5] as u8,
            mapper1: header_bytes[6] as u8,
            mapper2: header_bytes[7] as u8,
            prg_ram_size: header_bytes[8] as u8,
            tv_system1: header_bytes[9] as u8,
            tv_system2: header_bytes[10] as u8,
        };
        // Process the header and load PRG and CHR ROMs
        // This is a simplified example; actual implementation may vary
        println!("PRG ROM Size: {} KB", header.prg_rom_size * 16);
        println!("CHR ROM Size: {} KB", header.chr_rom_size * 8);

        if header.mapper1 & 0x04 != 0 {
            file.seek(SeekFrom::Current(512))?;
        }

        let mapper: u8 = (header.mapper1 >> 4) | (header.mapper2 & 0xF0);

        let mirroring = if header.mapper1 & 0b1000 != 0 {
            ScreenMirroring::FourScreen
        } else if header.mapper1 & 0b1 != 0 {
            ScreenMirroring::Vertical
        } else {
            ScreenMirroring::Horizontal
        };

        let mut prg_rom = vec![0u8; (header.prg_rom_size as usize) * 16 * 1024];

        file.read_exact(&mut prg_rom)?;

        let mut chr_rom: Vec<u8>;
        if header.chr_rom_size > 0 {
            chr_rom = vec![0u8; (header.chr_rom_size as usize) * 8 * 1024];
        } else {
            // If CHR ROM size is 0, we can use CHR RAM
            chr_rom = vec![0u8; 8192]; // 8 KB of CHR RAM
        }
        file.read_exact(&mut chr_rom)?;

        let mapper = match mapper {
            0 => Box::new(Mapper0::new(
                prg_rom.clone(),
                chr_rom.clone(),
                header.chr_rom_size == 0,
            )),
            _ => panic!("Unsupported mapper: {}", mapper),
        };

        println!("Mapper: {:?}", mapper.chr);

        self.mapper = mapper;
        self.mirroring = mirroring;
        Ok(())
    }

    pub fn reset(&mut self) {
        // Reset the cartridge state if needed
        // For now, we just reset the mapper
        self.mapper.reset();
    }
}
