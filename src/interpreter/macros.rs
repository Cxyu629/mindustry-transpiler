macro_rules! bin_err_msg {
    ($op: expr, $operand1: expr, $operand2: expr) => {
        format!(
            "Operand types `{}` and `{}` are invalid for operator '{}'.",
            $operand1.dtype(),
            $operand2.dtype(),
            $op.lexeme,
        )
    };
}

macro_rules! bin_match {
    ($operator:ident, $left:ident, $right: ident, $op:tt, {[$a1:tt, $b1:tt, $c1:tt]$(, [$a2:tt, $b2:tt, $c2:tt])* $(, )?}$(, {$([$pat:pat, $e:expr], )*})? ) => {
        match (&$left, &$right) {
            ($a1(val_left), $b1(val_right)) => Ok($c1(val_left.$op(val_right))),
            $(
                ($a2(val_left), $b2(val_right)) => Ok($c2((val_left).$op(val_right))),
            )*
            $($(
                $pat => $e,
            )*)?
            _ => generic_bin_error!($operator, $left, $right),
        }
    };
}

macro_rules! bin_match_deref {
    ($operator:ident, $left:ident, $right: ident, $op:tt, {[$a1:tt, $b1:tt, $c1:tt]$(, [$a2:tt, $b2:tt, $c2:tt])* $(, )?}$(, {$([$pat:pat, $e:expr], )*})? ) => {
        match (&$left, &$right) {
            ($a1(val_left), $b1(val_right)) => Ok($c1(val_left.$op(*val_right))),
            $(
                ($a2(val_left), $b2(val_right)) => Ok($c2((val_left).$op(*val_right))),
            )*
            $($(
                $pat => $e,
            )*)?
            _ => generic_bin_error!($operator, $left, $right),
        }
    };
}

macro_rules! bin_match_iuf {
    ($self:ident, $left:ident, $right: ident, $op:tt) => {
        match (&$left, &$right) {
            (Number(val_left), Number(val_right)) => {
                Ok(Number(((*val_left as i32).$op(*val_right as u32)) as f32))
            }
            _ => generic_bin_error!($self, $left, $right),
        }
    };
}

macro_rules! bin_match_iif {
    ($operator:ident, $left:ident, $right: ident, $op:tt) => {
        match (&$left, &$right) {
            (Number(val_left), Number(val_right)) => {
                Ok(Number(((*val_left as i32).$op(*val_right as i32)) as f32))
            }
            _ => generic_bin_error!($operator, $left, $right),
        }
    };
}

macro_rules! generic_bin_error {
    ($operator: ident, $left: ident, $right: ident) => {
        Err(Interpreter::error(
            &$operator,
            bin_err_msg!($operator, $left, $right),
        ))
    };
}

macro_rules! make_params {
    [] => {
        
    };
}