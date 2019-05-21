use crate::cpu::{
    ADD, ADDRESS, AND, B, CIR, CMP, COND, COND_EQ, COND_GT, COND_LT, COND_NE, COND_NONE, COUNTER,
    DATA, DEST, EOR, HALT, IMMEDIATE, INDIRECT, LDR, LSL, LSR, MBUFF, MOV, MVN, OFFSET, OPERATION,
    ORR, SHIFT, SOURCE, STATUS, STR, SUB,
};
use crate::storage::Storage;
use crate::stream::Error;

fn decode(n: u32, regs: &mut dyn Storage) -> Result<u32, Error> {
    let shift = (n & SHIFT) >> 8;
    let data = n & DATA;
    if n & IMMEDIATE == 0 {
        regs.get(data)
    } else {
        Ok(data.rotate_right(shift * 2))
    }
}

pub fn execute(memory: &mut dyn Storage, regs: &mut dyn Storage) -> Result<bool, Error> {
    regs.set(ADDRESS, regs.get(COUNTER)?)?;
    regs.set(COUNTER, regs.get(COUNTER)? + 1)?;

    regs.set(MBUFF, memory.get(regs.get(ADDRESS)?)?)?;

    let cir = regs.get(MBUFF)?;
    regs.set(CIR, cir)?;

    match cir & OPERATION {
        LDR => {
            let src = (cir & SOURCE) >> 16;
            let mem = cir & OFFSET;

            if cir & INDIRECT == 0 {
                regs.set(ADDRESS, mem)?;
            } else {
                regs.set(ADDRESS, regs.get(mem)?)?;
            }
            regs.set(MBUFF, memory.get(regs.get(ADDRESS)?)?)?;

            regs.set(src, regs.get(MBUFF)?)?;
        }
        STR => {
            let src = (cir & SOURCE) >> 16;
            let mem = cir & OFFSET;

            if cir & INDIRECT == 0 {
                regs.set(ADDRESS, mem)?;
            } else {
                regs.set(ADDRESS, regs.get(mem)?)?;
            }
            regs.set(MBUFF, regs.get(src)?)?;

            memory.set(regs.get(ADDRESS)?, regs.get(MBUFF)?)?;
        }
        ADD => {
            let src = (cir & SOURCE) >> 16;
            let dest = (cir & DEST) >> 12;
            let operand = decode(cir, regs)?;

            regs.set(dest, regs.get(src)?.wrapping_add(operand))?;
        }
        SUB => {
            let src = (cir & SOURCE) >> 16;
            let dest = (cir & DEST) >> 12;
            let operand = decode(cir, regs)?;

            regs.set(dest, regs.get(src)?.wrapping_sub(operand))?;
        }
        MOV => {
            let dest = (cir & DEST) >> 12;
            let operand = decode(cir, regs)?;

            regs.set(dest, operand)?;
        }
        CMP => {
            let src = (cir & SOURCE) >> 16;
            let operand = decode(cir, regs)?;

            let val = regs.get(src)?;
            regs.set(
                STATUS,
                if val == operand {
                    COND_EQ
                } else if val < operand {
                    COND_LT
                } else {
                    COND_GT
                },
            )?;
        }
        B => {
            let offset = cir & OFFSET;
            match cir & COND {
                COND_NONE => {
                    regs.set(COUNTER, offset)?;
                }
                cond => {
                    let status = regs.get(STATUS)? & COND;
                    if cond == status || (cond == COND_NE && status != COND_EQ) {
                        regs.set(COUNTER, offset)?;
                    }
                }
            }
        }
        AND => {
            let src = (cir & SOURCE) >> 16;
            let dest = (cir & DEST) >> 12;
            let operand = decode(cir, regs)?;

            regs.set(dest, regs.get(src)? & operand)?;
        }
        ORR => {
            let src = (cir & SOURCE) >> 16;
            let dest = (cir & DEST) >> 12;
            let operand = decode(cir, regs)?;

            regs.set(dest, regs.get(src)? | operand)?;
        }
        EOR => {
            let src = (cir & SOURCE) >> 16;
            let dest = (cir & DEST) >> 12;
            let operand = decode(cir, regs)?;

            regs.set(dest, regs.get(src)? ^ operand)?;
        }
        MVN => {
            let dest = (cir & DEST) >> 12;
            let operand = decode(cir, regs)?;

            regs.set(dest, !operand)?;
        }
        LSR => {
            let src = (cir & SOURCE) >> 16;
            let dest = (cir & DEST) >> 12;
            let operand = decode(cir, regs)?;

            regs.set(dest, regs.get(src)? >> operand)?;
        }
        LSL => {
            let src = (cir & SOURCE) >> 16;
            let dest = (cir & DEST) >> 12;
            let operand = decode(cir, regs)?;

            regs.set(dest, regs.get(src)? << operand)?;
        }
        HALT => return Ok(false),
        _ => return Err(Error::new(String::from("Unknown instruction"), None)),
    }
    Ok(true)
}
