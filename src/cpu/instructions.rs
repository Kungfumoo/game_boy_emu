use super::CPU;

pub struct StateChange {
    pub byte_length: u8,
    pub t_states: u8,
    pub flags: FlagChange,
    pub register: RegisterChange
}

pub struct RegisterChange {
    pub a: Option<u8>,
    pub b: Option<u8>,
    pub c: Option<u8>,
    pub d: Option<u8>,
    pub e: Option<u8>,
    pub f: Option<u8>,
    pub h: Option<u8>,
    pub l: Option<u8>
}

impl RegisterChange {
    fn default() -> RegisterChange {
        RegisterChange {
            a: Option::None,
            b: Option::None,
            c: Option::None,
            d: Option::None,
            e: Option::None,
            f: Option::None,
            h: Option::None,
            l: Option::None
        }
    }
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
        0x76 => halt(),
        _ => StateChange {
            byte_length: 0,
            t_states: 0,
            flags: FlagChange::default(),
            register: RegisterChange::default()
        }
    }
}

fn halt() -> StateChange {
    nop() //TODO: is this the same as nop?
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
        },
        register: RegisterChange::default()
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
        flags: FlagChange::default(),
        register: RegisterChange::default()
    }
}
