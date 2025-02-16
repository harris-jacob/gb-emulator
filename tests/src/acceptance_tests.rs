use crate::support::BlarggTestCase;
use crate::support::MooneyeTestCase;

struct CpuInstrs;
struct InstrTiming;
struct Tim00;
struct Tim01;
struct Tim11;
struct Tim10;
struct Tim00DivTrigger;
struct Tim01DivTrigger;
struct Tim10DivTrigger;
struct Tim11DivTrigger;

impl BlarggTestCase for CpuInstrs {
    fn filepath() -> String {
        "../roms/cpu_instrs/cpu_instrs.gb".to_string()
    }

    fn expected_output() -> String {
        "cpu_instrs\n\n01:ok  02:ok  03:ok  04:ok  05:ok  06:ok  07:ok  08:ok  09:ok  10:ok  11:ok  \n\nPassed all tests\n"
            .to_string()
    }

    fn steps() -> u32 {
        60000000
    }
}

impl BlarggTestCase for InstrTiming {
    fn filepath() -> String {
        "../roms/instr_timing.gb".to_string()
    }
    fn expected_output() -> String {
        "instr_timing\n\n\nPassed\n".to_string()
    }

    fn steps() -> u32 {
        1000000
    }
}

impl MooneyeTestCase for Tim01 {
    fn filepath() -> String {
        "../roms/acceptance/timer/tim01.gb".to_string()
    }

    fn steps() -> u32 {
        200000
    }
}

impl MooneyeTestCase for Tim10 {
    fn filepath() -> String {
        "../roms/acceptance/timer/tim10.gb".to_string()
    }

    fn steps() -> u32 {
        200000
    }
}

impl MooneyeTestCase for Tim11 {
    fn filepath() -> String {
        "../roms/acceptance/timer/tim10.gb".to_string()
    }

    fn steps() -> u32 {
        200000
    }
}

impl MooneyeTestCase for Tim00 {
    fn filepath() -> String {
        "../roms/acceptance/timer/tim00.gb".to_string()
    }

    fn steps() -> u32 {
        200000
    }
}

impl MooneyeTestCase for Tim00DivTrigger {
    fn filepath() -> String {
        "../roms/acceptance/timer/tim00_div_trigger.gb".to_string()
    }

    fn steps() -> u32 {
        200000
    }
}

impl MooneyeTestCase for Tim01DivTrigger {
    fn filepath() -> String {
        "../roms/acceptance/timer/tim01_div_trigger.gb".to_string()
    }

    fn steps() -> u32 {
        200000
    }
}

impl MooneyeTestCase for Tim10DivTrigger {
    fn filepath() -> String {
        "../roms/acceptance/timer/tim10_div_trigger.gb".to_string()
    }

    fn steps() -> u32 {
        200000
    }
}

impl MooneyeTestCase for Tim11DivTrigger {
    fn filepath() -> String {
        "../roms/acceptance/timer/tim11_div_trigger.gb".to_string()
    }

    fn steps() -> u32 {
        200000
    }
}

#[test]
fn cpu_instrs() {
    CpuInstrs::run();
}

#[test]
fn instr_timing() {
    InstrTiming::run();
}

#[test]
fn timer() {
    Tim00::run();
    Tim01::run();
    Tim10::run();
    Tim11::run();
    Tim00DivTrigger::run();
    Tim01DivTrigger::run();
    Tim10DivTrigger::run();
    Tim11DivTrigger::run();
}
