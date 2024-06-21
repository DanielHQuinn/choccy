use super::emulator::Emu;
use super::opcode::OpCode;

fn setup() -> Emu {
    let mut emu = Emu::new();
    emu.psuedo_registers.program_counter = 0; // just so we start with the same state
    emu
}

#[test]
fn test_opcode_nop() {
    let mut emu = setup();
    emu.ram[0] = 0x00;
    emu.ram[1] = 0x00;

    let opcode = emu.fetch_opcode();
    assert_eq!(opcode, OpCode::Nop);
}

#[test]
#[should_panic = "DEPRECATED!"]
fn test_opcode_call() {
    let mut emu = setup();

    emu.ram[0] = 0x02;
    emu.ram[1] = 0x34;

    let opcode = emu.fetch_opcode();

    assert_eq!(opcode, OpCode::Call(0x234));

    emu.execute_opcode(&opcode);
}

#[test]
fn test_opcode_return() {
    let mut emu = setup();

    emu.push_stack(0x200);

    emu.ram[0] = 0x00;
    emu.ram[1] = 0xEE;

    let opcode = emu.fetch_opcode();

    assert_eq!(opcode, OpCode::Return);

    emu.execute_opcode(&opcode);

    assert_eq!(emu.psuedo_registers.program_counter, 0x200);
}

#[test]
fn test_opcode_flow_jump() {
    let mut emu = setup();

    emu.ram[0] = 0x12;
    emu.ram[1] = 0x34;

    let opcode = emu.fetch_opcode();

    assert_eq!(opcode, OpCode::Flow(1, 0x234));

    emu.execute_opcode(&opcode);

    assert_eq!(emu.psuedo_registers.program_counter, 0x234);
}

#[test]
fn test_opcode_flow_call() {
    let mut emu = setup();

    emu.ram[0] = 0x23;
    emu.ram[1] = 0x45;

    let opcode = emu.fetch_opcode();

    assert_eq!(opcode, OpCode::Flow(2, 0x345));

    emu.execute_opcode(&opcode);

    assert_eq!(emu.psuedo_registers.program_counter, 0x345);
    let sp = emu.stack_pointer();
    assert_eq!(sp, 1);
    assert_eq!(emu.stack[sp as usize], 0);
}

#[test]
fn test_opcode_flow_jump_v0() {
    let mut emu = setup();

    emu.set_register_val(0, 0x12);

    emu.ram[0] = 0xB3;
    emu.ram[1] = 0x45;

    let opcode = emu.fetch_opcode();

    assert_eq!(opcode, OpCode::Flow(11, 0x345));

    emu.execute_opcode(&opcode);

    assert_eq!(emu.psuedo_registers.program_counter, 0x357);
}

#[test]
fn test_opcode_skip_equals() {
    let mut emu = setup();

    emu.set_register_val(0, 0x12);

    emu.ram[0] = 0x30;
    emu.ram[1] = 0x12;

    let opcode = emu.fetch_opcode();

    assert_eq!(opcode, OpCode::SkipEquals((3, 0, 0x12)));

    emu.execute_opcode(&opcode);

    assert_eq!(emu.psuedo_registers.program_counter, 4);
}

#[test]
fn test_opcode_skip_not_equals() {
    let mut emu = setup();

    emu.set_register_val(0, 0x12);

    emu.ram[0] = 0x40;
    emu.ram[1] = 0x34;

    let opcode = emu.fetch_opcode();

    assert_eq!(opcode, OpCode::SkipEquals((4, 0, 0x34)));

    emu.execute_opcode(&opcode);

    assert_eq!(emu.psuedo_registers.program_counter, 4);
}
#[test]
fn test_opcode_skip_register_equals() {
    let mut emu = setup();

    emu.set_register_val(0, 0x12);
    emu.set_register_val(1, 0x12);

    emu.ram[0] = 0x50;
    emu.ram[1] = 0x10;

    let opcode = emu.fetch_opcode();

    assert_eq!(opcode, OpCode::SkipRegEquals((5, 0, 1)));

    emu.execute_opcode(&opcode);

    assert_eq!(emu.psuedo_registers.program_counter, 4);
}

#[test]
fn test_opcode_skip_register_not_equals() {
    let mut emu = setup();

    emu.set_register_val(0, 0x12);
    emu.set_register_val(1, 0x34);

    emu.ram[0] = 0x90;
    emu.ram[1] = 0x10;

    let opcode = emu.fetch_opcode();

    assert_eq!(opcode, OpCode::SkipRegEquals((9, 0, 1)));

    emu.execute_opcode(&opcode);

    assert_eq!(emu.psuedo_registers.program_counter, 4);
}

#[test]
fn test_opcode_set_const() {
    let mut emu = setup();
    emu.set_register_val(0, 0x12);
    emu.ram[0] = 0x60;
    emu.ram[1] = 0x34;
    let opcode = emu.fetch_opcode();
    assert_eq!(opcode, OpCode::Constant((6, 0, 0x34)));
    emu.execute_opcode(&opcode);
    assert_eq!(emu.get_register_val(0), 0x34);
}

#[test]
fn test_opcode_add_const() {
    let mut emu = setup();
    emu.set_register_val(0, 0x12);
    emu.ram[0] = 0x70;
    emu.ram[1] = 0x34;
    let opcode = emu.fetch_opcode();
    assert_eq!(opcode, OpCode::Constant((7, 0, 0x34)));
    emu.execute_opcode(&opcode);
    assert_eq!(emu.get_register_val(0), 0x46);
}

//TODO: FIX BITOP TESTS
#[test]
fn test_opcode_bit_op0() {
    let mut emu = setup();
    emu.set_register_val(0, 0x12);
    emu.set_register_val(1, 0x34);
    emu.ram[0] = 0x80;
    emu.ram[1] = 0x10;
    let opcode = emu.fetch_opcode();
    assert_eq!(opcode, OpCode::BitOp((0, 1, 0)));
    emu.execute_opcode(&opcode);
    assert_eq!(emu.get_register_val(0), 0x34);
}

#[test]
fn test_opcode_bit_op1() {
    let mut emu = setup();
    emu.set_register_val(0, 0x12);
    emu.set_register_val(1, 0x36);
    emu.ram[0] = 0x80;
    emu.ram[1] = 0x11;
    let opcode = emu.fetch_opcode();
    assert_eq!(opcode, OpCode::BitOp((0, 1, 1)));
    emu.execute_opcode(&opcode);
    assert_eq!(emu.get_register_val(0), 0x36);
}

#[test]
fn test_opcode_bit_op2() {
    let mut emu = setup();
    emu.set_register_val(0, 0x12);
    emu.set_register_val(1, 0x12);
    emu.ram[0] = 0x80;
    emu.ram[1] = 0x12;
    let opcode = emu.fetch_opcode();
    assert_eq!(opcode, OpCode::BitOp((0, 1, 2)));
    emu.execute_opcode(&opcode);
    assert_eq!(emu.get_register_val(0), 0x12);
}

#[test]
fn test_opcode_bit_op3() {
    let mut emu = setup();
    emu.set_register_val(0, 0x12);
    emu.set_register_val(1, 0x34);
    emu.ram[0] = 0x80;
    emu.ram[1] = 0x13;
    let opcode = emu.fetch_opcode();
    assert_eq!(opcode, OpCode::BitOp((0, 1, 3)));
    emu.execute_opcode(&opcode);
    assert_eq!(emu.get_register_val(0), 0x26);
}

#[test]
fn test_opcode_bit_op4() {
    let mut emu = setup();
    emu.set_register_val(0, 0x12);
    emu.set_register_val(1, 0x34);
    emu.ram[0] = 0x80;
    emu.ram[1] = 0x14;
    let opcode = emu.fetch_opcode();
    assert_eq!(opcode, OpCode::BitOp((0, 1, 4)));
    emu.execute_opcode(&opcode);
    assert_eq!(emu.get_register_val(0), 0x46);
}

#[test]
fn test_opcode_bit_op5() {
    let mut emu = setup();
    emu.set_register_val(0, 0x20);
    emu.set_register_val(1, 0x10);
    emu.ram[0] = 0x80;
    emu.ram[1] = 0x15;
    let opcode = emu.fetch_opcode();
    assert_eq!(opcode, OpCode::BitOp((0, 1, 5)));
    emu.execute_opcode(&opcode);
    assert_eq!(emu.get_register_val(0), 0x10);
}

#[test]
fn test_opcode_bit_op6() {
    let mut emu = setup();
    emu.set_register_val(0, 0x12);
    emu.set_register_val(1, 0x00);
    emu.ram[0] = 0x80;
    emu.ram[1] = 0x16;
    let opcode = emu.fetch_opcode();
    assert_eq!(opcode, OpCode::BitOp((0, 1, 6)));
    emu.execute_opcode(&opcode);
    assert_eq!(emu.get_register_val(0), 0x09);
}

#[test]
fn test_opcode_bit_op7() {
    let mut emu = setup();
    emu.set_register_val(0, 0x12);
    emu.set_register_val(1, 0x34);
    emu.ram[0] = 0x80;
    emu.ram[1] = 0x17;
    let opcode = emu.fetch_opcode();
    assert_eq!(opcode, OpCode::BitOp((0, 1, 7)));
    emu.execute_opcode(&opcode);
    assert_eq!(emu.get_register_val(0), 0x22);
}

#[test]
fn test_opcode_bit_ope() {
    let mut emu = setup();
    emu.set_register_val(0, 0x12);
    emu.set_register_val(1, 0x00);
    emu.ram[0] = 0x80;
    emu.ram[1] = 0x1E;
    let opcode = emu.fetch_opcode();
    assert_eq!(opcode, OpCode::BitOp((0, 1, 0xE)));
    emu.execute_opcode(&opcode);
    assert_eq!(emu.get_register_val(0), 0x24);
}

#[test]
fn test_opcode_iop() {
    let mut emu = setup();

    emu.ram[0] = 0xA2;
    emu.ram[1] = 0x34;

    let opcode = emu.fetch_opcode();

    assert_eq!(opcode, OpCode::IOp(0x234));

    emu.execute_opcode(&opcode);

    assert_eq!(emu.i_register, 0x234);
}

#[test]
fn test_opcode_memory_op1e() {
    let mut emu = setup();

    emu.set_register_val(0, 0x12);
    emu.i_register = 0x34;

    emu.ram[0] = 0xF0;
    emu.ram[1] = 0x1E;

    let opcode = emu.fetch_opcode();

    assert_eq!(opcode, OpCode::MemoryOp((0, 0x1E)));

    emu.execute_opcode(&opcode);

    assert_eq!(emu.i_register, 0x46);

    emu.set_register_val(0, 0x1);

    emu.i_register = 0xFFFF; // this can be upto 0xFFFF

    emu.execute_opcode(&opcode);

    assert_eq!(emu.i_register, 0x0);
}

#[test]
fn test_opcode_memory_op29() {
    let mut emu = setup();

    emu.set_register_val(0, 0x1);

    emu.ram[0] = 0xF0;
    emu.ram[1] = 0x29;

    let opcode = emu.fetch_opcode();

    assert_eq!(opcode, OpCode::MemoryOp((0, 29))); // here 29 is just 29 and not 0x29

    emu.execute_opcode(&opcode);

    assert_eq!(emu.i_register, 0x5);
}

#[test]
fn test_opcode_memory_op55() {
    let mut emu = setup();

    emu.set_register_val(0, 0x1);
    emu.set_register_val(1, 0x2);
    emu.set_register_val(2, 0x3);
    emu.set_register_val(3, 0x4);

    emu.i_register = 0x34;

    emu.ram[0] = 0xF3;
    emu.ram[1] = 0x55;

    let opcode = emu.fetch_opcode();

    assert_eq!(opcode, OpCode::MemoryOp((3, 55))); // here 55 is just 55 and not 0x55

    emu.execute_opcode(&opcode);

    // now, the following are in memory
    assert_eq!(emu.ram[0x34], 0x1);
    assert_eq!(emu.ram[0x35], 0x2);
    assert_eq!(emu.ram[0x36], 0x3);
    assert_eq!(emu.ram[0x37], 0x4);
}

#[test]
fn test_opcode_memory_op65() {
    let mut emu = setup();

    emu.i_register = 0x34;

    emu.ram[0] = 0xF3;
    emu.ram[1] = 0x65;

    emu.ram[0x34] = 0x1;
    emu.ram[0x35] = 0x2;
    emu.ram[0x36] = 0x3;
    emu.ram[0x37] = 0x4;

    let opcode = emu.fetch_opcode();

    assert_eq!(opcode, OpCode::MemoryOp((3, 65))); // here 65 is just 65 and not 0x65

    emu.execute_opcode(&opcode);

    assert_eq!(emu.get_register_val(0), 0x1);
    assert_eq!(emu.get_register_val(1), 0x2);
    assert_eq!(emu.get_register_val(2), 0x3);
    assert_eq!(emu.get_register_val(3), 0x4);
}

#[test]
fn test_opcode_keyop_skip_equals() {
    let mut emu = setup();

    emu.set_register_val(0, 0x1);
    emu.keys[0x1] = true;

    emu.ram[0] = 0xE0;
    emu.ram[1] = 0x9E;

    let opcode = emu.fetch_opcode();

    assert_eq!(opcode, OpCode::KeyOpSkip(0x9E, 0));

    emu.execute_opcode(&opcode);

    assert_eq!(emu.psuedo_registers.program_counter, 4);
}

#[test]
fn test_opcode_keyop_skip_not_equals() {
    let mut emu = setup();

    emu.set_register_val(0, 0x1);
    emu.keys[0x1] = false;

    emu.ram[0] = 0xE0;
    emu.ram[1] = 0xA1;

    let opcode = emu.fetch_opcode();

    assert_eq!(opcode, OpCode::KeyOpSkip(0xA1, 0));

    emu.execute_opcode(&opcode);

    assert_eq!(emu.psuedo_registers.program_counter, 4);
}

#[test]
fn test_set_delay_timer() {
    let mut emu = setup();

    emu.set_register_val(0, 0x1);

    emu.ram[0] = 0xF0;
    emu.ram[1] = 0x15;

    let opcode = emu.fetch_opcode();
    assert_eq!(opcode, OpCode::Timer((0, 5)));

    emu.execute_opcode(&opcode);

    assert_eq!(emu.get_register_val(0), emu.get_delay_timer());
}

#[test]
fn test_sound_timer() {
    let mut emu = setup();

    emu.set_register_val(0, 0x1);

    emu.ram[0] = 0xF0;
    emu.ram[1] = 0x18;

    let opcode = emu.fetch_opcode();
    assert_eq!(opcode, OpCode::Timer((0, 8)));

    emu.execute_opcode(&opcode);

    assert_eq!(emu.get_register_val(0), emu.get_sound_timer());
}

#[test]
fn test_sound_delay_timer() {
    let mut emu = setup();

    emu.set_delay_timer(0x1);

    emu.ram[0] = 0xF0;
    emu.ram[1] = 0x07;

    let opcode = emu.fetch_opcode();
    assert_eq!(opcode, OpCode::Timer((0, 7)));

    emu.execute_opcode(&opcode);

    assert_eq!(emu.get_register_val(0), emu.get_delay_timer());
}

#[test]
fn test_opcode_rand() {
    let mut emu = setup();

    emu.ram[0] = 0xC0;
    emu.ram[1] = 0x12;

    let opcode = emu.fetch_opcode();

    assert_eq!(opcode, OpCode::RandomOp((0, 0x12)));

    emu.execute_opcode(&opcode);

    let register_val = emu.get_register_val(0);

    println!("Register 0: {register_val}");
}

#[test]
fn test_opcode_display() {
    let mut emu = setup();

    // first,, we clear the screen
    emu.screen.fill(true);

    emu.ram[0] = 0x00;
    emu.ram[1] = 0xE0;

    let opcode = emu.fetch_opcode();
    assert_eq!(opcode, OpCode::Display(None));

    emu.execute_opcode(&opcode);
    assert!(emu.screen.iter().all(|&x| !x));

    // now we draw a sprite
    emu.set_register_val(0, 0);
    emu.set_register_val(1, 0);
    emu.set_register_val(0xF, 0);

    emu.set_program_counter(0x0);

    emu.i_register = 0x0;
    emu.ram[0] = 0xD0;
    emu.ram[1] = 0x15;

    let opcode = emu.fetch_opcode();
    assert_eq!(opcode, OpCode::Display(Some((0, 1, 5))));
}

#[test]
fn test_opcode_bcd() {
    let mut emu = setup();

    emu.set_register_val(0, 123);

    emu.ram[0] = 0xF0;
    emu.ram[1] = 0x33;

    let opcode = emu.fetch_opcode();

    assert_eq!(opcode, OpCode::Bcd(0));

    emu.execute_opcode(&opcode);

    let i_reg = emu.i_register as usize;

    assert_eq!(emu.ram[i_reg], 1);
    assert_eq!(emu.ram[i_reg + 1], 2);
    assert_eq!(emu.ram[i_reg + 2], 3);
}

#[test]
fn test_op_code() {
    let mut emu = setup();

    let opcodes = [
        0x60, 0x01, // 0x6001 // set register 0 to 1
        0x81, 0x00, // 0x8100 // set register 1 to the val of register 0
        0x70, 0x02, // 0x7002 // add 2 to register 0
        0x90, 0x10, // 0x9010 // skip next instruction if reg 0 is neq to reg 1
        0x00, 0x1E, // 0x00EE // this should be 'call' which is deprecated <- else panic
        0x80, 0x14, // 0x8014 // increment register 0 by register 1
        0x6e, 0xff, // 0x6F00 // set register 0xF to 255
        0x7e, 0x00, // 0x7F01 // add 1 to register 0xF
        0x8e, 0x14, // 0x8E01 // set register 0xE to 1
    ];

    emu.ram[0..opcodes.len()].copy_from_slice(&opcodes);

    let first_op = emu.fetch_opcode();
    assert_eq!(first_op, OpCode::Constant((6, 0, 1)));
    emu.execute_opcode(&first_op);
    assert_eq!(emu.get_register_val(0), 1);
    assert_eq!(emu.program_counter(), 2);

    let second_op = emu.fetch_opcode();
    assert_eq!(second_op, OpCode::BitOp((1, 0, 0)));
    emu.execute_opcode(&second_op);
    assert_eq!(emu.get_register_val(1), 1);
    assert_eq!(emu.program_counter(), 4);

    let third_op = emu.fetch_opcode();
    assert_eq!(third_op, OpCode::Constant((7, 0, 2)));
    emu.execute_opcode(&third_op);
    assert_eq!(emu.get_register_val(0), 3);
    assert_eq!(emu.program_counter(), 6);

    let fourth_op = emu.fetch_opcode();
    assert_eq!(fourth_op, OpCode::SkipRegEquals((9, 0, 1)));
    emu.execute_opcode(&fourth_op);
    assert_eq!(emu.program_counter(), 10); // cause we skip to the next instruction

    let fifth_op = emu.fetch_opcode();
    assert_eq!(fifth_op, OpCode::BitOp((0, 1, 4)));
    emu.execute_opcode(&fifth_op);
    assert_eq!(emu.get_register_val(0), 4);
    assert_eq!(emu.program_counter(), 12);
    assert_eq!(emu.get_register_val(0xf), 0);

    let sixth_op = emu.fetch_opcode();
    assert_eq!(sixth_op, OpCode::Constant((6, 0xe, 0xff)));
    emu.execute_opcode(&sixth_op);
    assert_eq!(emu.get_register_val(0xe), 0xff);
    assert_eq!(emu.program_counter(), 14);

    let seventh_op = emu.fetch_opcode();
    assert_eq!(seventh_op, OpCode::Constant((7, 0xe, 0)));
    emu.execute_opcode(&seventh_op);
    assert_eq!(emu.get_register_val(0xe), 0xff);
    assert_eq!(emu.program_counter(), 16);
    assert_eq!(emu.get_register_val(0xf), 0); // here f is 0

    let eighth_op = emu.fetch_opcode();
    assert_eq!(eighth_op, OpCode::BitOp((14, 1, 4)));
    emu.execute_opcode(&eighth_op);
    assert_eq!(emu.get_register_val(0xe), 0);
    assert_eq!(emu.program_counter(), 18);
    assert_eq!(emu.get_register_val(0xf), 1); // now f is 1 since we overflowed
}

#[test]
fn test_opcode_keyop_wait() {
    let mut emu = setup();

    emu.keys[0] = true;

    emu.ram[0] = 0xF0;
    emu.ram[1] = 0x0A;

    let opcode = emu.fetch_opcode();

    assert_eq!(opcode, OpCode::KeyOpWait(0));

    emu.execute_opcode(&opcode);

    assert_eq!(emu.get_register_val(0), 0);
}
