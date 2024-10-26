mod i8008;
use i8008::*;

use std::io;
use std::io::Write;

macro_rules! equiv_select {
    ($v1:expr, $v2:expr) => {{
        if $v1 == $v2 {
            "(*)"
        } else {
            "   "
        }
    }};
}

fn main() {
    let mut cpu_sim: I8008 = I8008::new();
    let mut mem_controller: I8008MemoryController = I8008MemoryController::new();
    // TODO: populate memory

    let mut address_low: u8 = 0x00;
    let mut address_high: u8 = 0x00;
    mem_controller.load_into(
        0x00,
        &[
            I8008Ins::INr as u8 | 0x08, // INb
            I8008Ins::DCr as u8 | 0x18, // INd
            I8008Ins::Lrr as u8 | 0x1C, // Lde
        ],
    );
    loop {
        print!(">");
        let _ = io::stdout().flush();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        match cpu_sim.get_state() {
            CpuState::T1 => {
                address_low = cpu_sim.databus;
            }
            CpuState::T2 => {
                address_high = cpu_sim.databus;
            }
            CpuState::T3 => {
                let address: u16 = (address_low as u16) + ((address_high as u16) << 8);
                cpu_sim.databus = mem_controller.get_value(address);
            }
            _ => (),
        }
        let mut words = input.split(" ");
        match words.next() {
            Some("state") => {
                println!("state: {:?}", cpu_sim.get_state());
            }
            Some("step") => {
                cpu_sim.step();
            }
            Some("address") => match input.split(" ").nth(1) {
                Some("low") => println!("{:#04X}", address_low),
                Some("high") => println!("{:#04X}", address_high),
                Some(&_) => panic!("error: unrecognized argument"),
                None => panic!("error: expected an argument"),
            },
            Some("set_line") => {
                let value: bool = match words.nth(2) {
                    Some("1") => true,
                    Some("high") => true,
                    Some("0") => false,
                    Some("low") => false,
                    Some(&_) => panic!("error: unexpected value"),
                    None => true,
                };
                match input.split(" ").nth(1) {
                    Some("interrupt") => cpu_sim.line_interrupt = value,
                    Some("ready") => cpu_sim.line_ready = value,
                    Some(&_) => panic!("error: unrecognized argument"),
                    None => panic!("error: expected an argument"),
                }
            }
            Some("databus") => match words.nth(1) {
                Some(val) => cpu_sim.databus = reverse_u8(val.parse::<u8>().unwrap()),
                None => panic!("error: expected an argument"),
            },
            Some("status") => {
                let sp: u8 = cpu_sim.get_stack_pointer();
                println!("stack:    scratchpad:  internal:");
                println!(
                    "A|{:#04X}{} A|{:#04X}       {:#04X}",
                    cpu_sim.get_stack_register(0),
                    equiv_select!(sp, 0),
                    cpu_sim.get_scratchpad_register(false, false, false),
                    cpu_sim.get_register_a()
                );
                println!(
                    "B|{:#04X}{} B|{:#04X}       {:#04X}",
                    cpu_sim.get_stack_register(1),
                    equiv_select!(sp, 1),
                    cpu_sim.get_scratchpad_register(false, false, true),
                    cpu_sim.get_register_b()
                );
                println!(
                    "C|{:#04X}{} C|{:#04X}",
                    cpu_sim.get_stack_register(2),
                    equiv_select!(sp, 2),
                    cpu_sim.get_scratchpad_register(false, true, false)
                );
                println!(
                    "D|{:#04X}{} D|{:#04X}",
                    cpu_sim.get_stack_register(3),
                    equiv_select!(sp, 3),
                    cpu_sim.get_scratchpad_register(false, true, true)
                );
                println!(
                    "E|{:#04X}{} E|{:#04X}",
                    cpu_sim.get_stack_register(4),
                    equiv_select!(sp, 4),
                    cpu_sim.get_scratchpad_register(true, false, false)
                );
                println!(
                    "F|{:#04X}{} L|{:#04X}",
                    cpu_sim.get_stack_register(5),
                    equiv_select!(sp, 5),
                    cpu_sim.get_scratchpad_register(true, false, true)
                );
                println!(
                    "G|{:#04X}{} H|{:#04X}",
                    cpu_sim.get_stack_register(6),
                    equiv_select!(sp, 6),
                    cpu_sim.get_scratchpad_register(true, true, false)
                );
                println!(
                    "H|{:#04X}{}",
                    cpu_sim.get_stack_register(7),
                    equiv_select!(sp, 7)
                );
                println!("");
                println!("flags:");
                if !cpu_sim.select_flag(false, false)
                    && !cpu_sim.select_flag(false, true)
                    && !cpu_sim.select_flag(true, false)
                    && !cpu_sim.select_flag(true, true)
                {
                    println!("(none set)")
                }
                if cpu_sim.select_flag(false, false) {
                    println!("CARRY")
                }
                if cpu_sim.select_flag(false, true) {
                    println!("ZERO")
                }
                if cpu_sim.select_flag(true, false) {
                    println!("SIGN")
                }
                if cpu_sim.select_flag(true, true) {
                    println!("PARITY")
                }
                println!();
                println!("cycle: {:?}", cpu_sim.get_cycle());
                println!("state: {:?}", cpu_sim.get_state());
                println!();
                println!("databus: {:#04X}", cpu_sim.databus);
                println!("ready line: {}", cpu_sim.line_ready);
                println!("interrupt line: {}", cpu_sim.line_interrupt);
            }
            Some("all") => println!("{:#?}", cpu_sim),
            Some("quit") => break,
            Some(&_) => {
                panic!("error: unknown command supplied");
            }
            None => (),
        }
    }
}
