use super::emulator::Emu;

fn setup() -> Emu {
    let mut emu = Emu::new();
    emu
}

#[test]
fn test_load_into_ram() {
    // This program, draws a line from (0, 0) to (63, 31)
    // Once the line is drawn, the program loops forever doing nothing.
    let rom: Vec<u8> = vec![
        0x12, 0x11, 0x80, 0xFF, 0x07, 0x3F, 0x00, 0x12, 0x03, 0x6F, 0x3C, 0xFF, 0x15, 0x00, 0xEE,
        0x12, 0x0F, 0xA2, 0x02, 0x60, 0x00, 0x61, 0x00, 0x40, 0x40, 0x22, 0x0F, 0xD0, 0x11, 0x70,
        0x01, 0x12, 0x17,
    ];

    let mut emu = setup();
    let _ = emu.load_rom(&rom);

    let segment = &emu.ram[0x200..0x200 + rom.len()];

    assert_eq!(segment, rom.as_slice());
}

#[test]
fn test_display_logic() {
    // This program, draws a line from (0, 0) to (63, 31)
    // Once the line is drawn, the program loops forever doing nothing.
    let rom: Vec<u8> = vec![
        0x12, 0x11, 0x80, 0xFF, 0x07, 0x3F, 0x00, 0x12, 0x03, 0x6F, 0x3C, 0xFF, 0x15, 0x00, 0xEE,
        0x12, 0x0F, 0xA2, 0x02, 0x60, 0x00, 0x61, 0x00, 0x40, 0x40, 0x22, 0x0F, 0xD0, 0x11, 0x70,
        0x01, 0x12, 0x17,
    ];
    let mut emu = setup();
    let _ = emu.load_rom(&rom);

    let mut stop_flag = emu.get_register_val(0);

    // while stop_flag < 10
    while stop_flag < 64 {
        let opcode = emu.fetch_opcode();
        match emu.execute_opcode(&opcode) {
            Ok(()) => {}
            Err(error) => {
                // exit while loop if error is error
                print!("Error: {error}");
            }
        }
        stop_flag = emu.get_register_val(0);
    }

    let screen = emu.screen;
    // Expected is vec of 2,048 bools, the first 64 are true, the rest are false
    let mut expected: Vec<bool> = vec![true; 64];
    expected.extend(vec![false; 1984]);

    assert!(screen == expected.as_slice());
    // Prints the screen after the program has run
    // let mut count = 0;
    // let mut rows = 0;
    // for val in &screen {
    //     print!("{}", if *val { "1" } else { "0" });
    //     count += 1;
    //     if count % 64 == 0 {
    //         println!();
    //         rows += 1;
    //     }
    // }
    // print!("Rows: {rows}");
    // print!("\nCols: 64");
}
