use super::CPU;

pub struct StateChange {
    pub byte_length: u8,
    pub t_states: u8,
    pub flags: FlagChange
}

pub struct FlagChange {
    pub zero: Option<bool>,
    pub subtract: Option<bool>,
    pub half_carry: Option<bool>,
    pub carry: Option<bool>
}

impl FlagChange {
    fn default() -> FlagChange {
        FlagChange {
            zero: Option::None,
            subtract: Option::None,
            half_carry: Option::None,
            carry: Option::None
        }
    }
}

pub fn execute(cpu: &CPU, op_code: u8) -> StateChange {
    match op_code {
        0x00 => nop(),
        0x10 => stop(),
        0x37 => scf(),
        _ => StateChange {
            byte_length: 0,
            t_states: 0,
            flags: FlagChange::default()
        }
    }
}

fn scf() -> StateChange {
    StateChange {
        byte_length: 1,
        t_states: 4,
        flags: FlagChange {
            carry: Some(true),
            subtract: Some(false),
            half_carry: Some(false),
            ..FlagChange::default()
        }
    }
}

fn stop() -> StateChange {
    StateChange {
        byte_length: 2,
        ..nop()
    }
}

fn nop() -> StateChange {
    //do nothing
    StateChange {
        byte_length: 1,
        t_states: 4,
        flags: FlagChange::default()
    }
}
