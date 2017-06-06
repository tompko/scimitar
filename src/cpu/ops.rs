#[macro_export]
macro_rules! read_u8 {
    ($b: expr, $a: expr) => {
        {
            let val = $b.interconnect.read_byte($a);
            $b.step(CYCLES_PER_STEP);
            val
        }
    }
}

#[macro_export]
macro_rules! read_pc_u8 {
    ($c: expr, $b: expr) => {
        {
            let val = $b.interconnect.read_byte($c.pc);
            $b.step(CYCLES_PER_STEP);

            if $c.halted == -1 {
                $c.halted = 0;
            } else {
                $c.pc += 1;
            }
            val
        }
    }
}

#[macro_export]
macro_rules! read_pc_u16 {
    ($c: expr, $b: expr) => {
        {
            let lsb = read_pc_u8!($c, $b);
            let msb = read_pc_u8!($c, $b);

            ((msb as u16) << 8) | (lsb as u16)
        }
    }
}

#[macro_export]
macro_rules! write_u8 {
    ( $b: expr, $a: expr, $v: expr) => {
        {
            $b.interconnect.write_byte($a, $v);
            $b.step(CYCLES_PER_STEP);
        }
    }
}

#[macro_export]
macro_rules! write_u16 {
    ($b: expr, $a: expr, $v: expr) => {
        {
            let msb = ($v >> 8) as u8;
            let lsb = ($v & 0xff) as u8;

            write_u8!($b, $a + 1, msb);
            write_u8!($b, $a, lsb);
        }
    }
}

#[macro_export]
macro_rules! push_u8 {
    ($c: expr, $b: expr, $v: expr) => {
        {
            $c.sp -= 1;
            write_u8!($b, $c.sp, $v);
        }
    }
}

#[macro_export]
macro_rules! push_u16 {
    ($c: expr, $b: expr, $v: expr) => {
        {
            let msb = ($v >> 8) as u8;
            let lsb = ($v & 0xff) as u8;
            $b.step(CYCLES_PER_STEP); // Internal delay
            push_u8!($c, $b, msb);
            push_u8!($c, $b, lsb);
        }
    }
}

#[macro_export]
macro_rules! pop_u8 {
    ($c: expr, $b: expr) => {
        {
            let val = read_u8!($b, $c.sp);
            $c.sp += 1;
            val
        }
    }
}

#[macro_export]
macro_rules! pop_u16 {
    ($c: expr, $b: expr) => {
        {
            let lsb = pop_u8!($c, $b) as u16;
            let msb = pop_u8!($c, $b) as u16;

            (msb << 8) | lsb
        }
    }
}

#[macro_export]
macro_rules! call {
    ($c: expr, $b: expr, $a: expr) => {
        {
            let pc = $c.pc;
            push_u16!($c, $b, pc);
            $c.pc = $a;
        }
    }
}

#[macro_export]
macro_rules! ret {
    ($c: expr, $b: expr) => {
        {
            let addr = pop_u16!($c, $b);
            $b.step(CYCLES_PER_STEP); // Internal delay
            $c.pc = addr;
        }
    }
}

#[macro_export]
macro_rules! rr {
    ($c: expr, $b: expr, $v: expr) => {
        {
            let carry = if $c.f.c { 1 } else { 0 };
            let ret = ($v >> 1) | (carry << 7);

            $c.f.z = ret == 0;
            $c.f.n = false;
            $c.f.h = false;
            $c.f.c = ($v & 0x01) != 0;

            $b.interconnect.step(CYCLES_PER_STEP, $b.device, $b.events);

            ret
        }
    }
}

#[macro_export]
macro_rules! nop {
    () => {{}}
}

#[macro_export]
macro_rules! inc {
    ($c: expr, $v: expr) => {
        {
            let r = $v.wrapping_add(1);

            $c.f.z = r == 0;
            $c.f.n = false;
            $c.f.h = (r & 0x0f) == 0;

            r
        }
    }
}
