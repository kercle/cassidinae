use std::fmt::{Debug, Formatter};

use crate::pattern::program::{ArgPlan, Instruction, Program};

impl<A: Clone + PartialEq + Debug> Debug for ArgPlan<A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        use ArgPlan::*;
        match self {
            Sequence(instructions) => {
                let mut leading_char = '[';
                for instr in instructions.iter() {
                    write!(f, "{leading_char}{instr}")?;
                    leading_char = ',';
                }
                if leading_char == '[' {
                    write!(f, "[]")
                } else {
                    write!(f, "]")
                }
            }
            Multiset(_) => todo!(),
        }
    }
}

impl<A: Clone + PartialEq + Debug> Debug for Instruction<A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        use Instruction::*;
        match self {
            Literal(e) => write!(f, "lit {e:?}"),
            Variadic {
                quantity,
                head_pattern,
            } => write!(f, "var quant={quantity:?} head={head_pattern:?}"),
            Node { head, plan } => {
                write!(f, "node head={head:?} plan={plan:?}")
            }
            Bind { variable, inner } => {
                write!(f, "bind var={variable} inner={inner}")
            }
            _ => todo!(),
        }
    }
}

impl<A: Clone + PartialEq + Debug> Debug for Program<A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "Program:\nEntry point: {:02}", self.entry)?;
        writeln!(f, "Vars:")?;
        for (idx, name) in self.vars.iter().enumerate() {
            writeln!(f, "  [{:02}] {}", idx, name)?;
        }
        writeln!(f, "Instructions:")?;
        for (idx, instr) in self.instructions.iter().enumerate() {
            writeln!(f, "  [{:02}] {:?}", idx, instr)?;
        }

        Ok(())
    }
}
