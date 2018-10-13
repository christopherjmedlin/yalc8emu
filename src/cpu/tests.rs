use super::*;

// helper function for automating rom loading
fn make_chip8_and_load_rom() -> Chip8 {
    let mut chip8 = Chip8::new();

    // just some arbitrary values for testing
    let rom: [u8; 7] = [
        0xFF, 0xFF, 0xFF, 0xFF, 0x80, 0x70, 0x10
    ];
    chip8.load_rom(&rom);

    chip8
}

#[test]
fn test_new_chip8() {
    let chip8 = Chip8::new();
    
    assert_eq!(chip8.i, 0);
    assert_eq!(chip8.stack.len(), 16);
    assert_eq!(chip8.v[0], 0);

    // test that some of the fonts are in the proper location
    assert_eq!(chip8.ram[0], 0xF0);
    assert_eq!(chip8.ram[9], 0x70);
    assert_eq!(chip8.ram[79], 0x80);
}

#[test]
fn test_load_rom() {
    let chip8 = make_chip8_and_load_rom();
    
    assert_eq!(chip8.ram[0x200], 0xFF);
    assert_eq!(chip8.ram[0x206], 0x10);
    assert_eq!(chip8.ram[0x207], 0);
}

#[test]
fn test_get_opcode() {
    let mut chip8 = make_chip8_and_load_rom();

    assert_eq!(chip8.get_opcode(), 0xFFFF);
}  

#[test]
fn test_2nnn_and_00EE() {
    let mut chip8 = Chip8::new();

    // jump to subroutine at 0x205
    chip8.run_opcode(0x2205);
    assert_eq!(chip8.pc, 0x205);
    assert_eq!(chip8.stack[1], 0x202);

    chip8.run_opcode(0x00EE);
    assert_eq!(chip8.pc, 0x202);
    assert_eq!(chip8.sp, 0);
}

#[test]
fn test_1nnn() {
    let mut chip8 = Chip8::new();

    chip8.run_opcode(0x1FFF);
    assert_eq!(chip8.pc, 0xFFF);
}

#[test]
fn test_3xkk_and_4xkk() {
    let mut chip8 = Chip8::new();
    chip8.v[0] = 2;
    
    // not equal
    chip8.run_opcode(0x3003);
    // shouldn't skip
    assert_eq!(chip8.pc, 0x202);
    
    // equal
    chip8.run_opcode(0x3002);
    // should skip
    assert_eq!(chip8.pc, 0x206);

    // not equal
    chip8.run_opcode(0x4003);
    // should skip
    assert_eq!(chip8.pc, 0x20a);

    // equal
    chip8.run_opcode(0x4002);
    // shouldn't skip
    assert_eq!(chip8.pc, 0x20c);
}

#[test]
fn test_5xy0() {
    let mut chip8 = Chip8::new();
    chip8.v[0] = 5;
    chip8.v[1] = 0;
    chip8.v[2] = 5;
    
    // not equal
    chip8.run_opcode(0x5010);
    // shouldn't skip
    assert_eq!(chip8.pc, 0x202);

    // equal
    chip8.run_opcode(0x5020);
    // should skip
    assert_eq!(chip8.pc, 0x206);
}

// macro for generating tests for operations that modify a register
// 
// $name is the name of the generated function
// $op is the opcode to be ran
// $r1 is the value initialized in v[1]
// $r2 is the value initialized in v[2]
// $val is the value that v[2] 
// $carry is the expected value of vF, aka the carry and borrow flag
macro_rules! test_register_op {
    ($name:ident, $op:expr, $r1:expr, $r2:expr, $val:expr, $carry:expr) => {
        #[test]
        fn $name() {
            let mut chip8 = Chip8::new();
            chip8.v[1] = $r1;
            chip8.v[2] = $r2;
            chip8.run_opcode($op);

            assert_eq!(chip8.v[2], $val);
            assert_eq!(chip8.v[0xF], $carry);
        }
    }
}

test_register_op!(test_6xkk, 0x620a, 0, 0, 10, 0);
test_register_op!(test_7xkk, 0x720a, 0, 10, 20, 0);
test_register_op!(test_8xy0, 0x8210, 5, 10, 5, 0);
test_register_op!(test_8xy1, 0x8211, 5, 20, 21, 0);
test_register_op!(test_8xy2, 0x8212, 5, 20, 4, 0);
test_register_op!(test_8xy3, 0x8213, 5, 20, 17, 0);
test_register_op!(test_8xy4, 0x8214, 5, 20, 25, 0);
test_register_op!(test_8xy4_carry, 0x8214, 200, 200, 144, 1);
test_register_op!(test_8xy5, 0x8215, 5, 20, 15, 1);
test_register_op!(test_8xy5_borrow, 0x8215, 20, 5, 241, 0);
test_register_op!(test_8xy6, 0x8216, 123, 20, 10, 0);
test_register_op!(test_8xy6_odd, 0x8216, 123, 21, 10, 1);
test_register_op!(test_8xy7, 0x8127, 5, 20, 15, 1);
test_register_op!(test_8xy7_borrow, 0x8127, 20, 5, 241, 0);
test_register_op!(test_8xy8, 0x8218, 123, 20, 40, 0);
test_register_op!(test_8xy8_odd, 0x8218, 123, 21, 42, 1);
