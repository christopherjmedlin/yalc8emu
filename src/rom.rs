use std::fs::File;
use std::path::Path;
use std::io::Read;

fn load_rom_file(path: &str, buf: &mut [u8]) {
    let path = Path::new(path);

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open ROM"),
        Ok(file) => file
    };

    file.read(buf);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_rom_file() {
        let mut rom = [0; 1000];
        load_rom_file("assets/maze.ch8", &mut rom);

        assert_eq!(rom[0], 0x30);
        assert_eq!(rom[0x10], 0x32);
    }
}
