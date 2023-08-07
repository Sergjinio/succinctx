use plonky2::iop::target::Target;

pub struct BoolVariable {
    pub value: Target,
}

pub impl BoolVariable {
    pub fn from_target(value: Target) -> Self {
        Self { value }
    }
}
