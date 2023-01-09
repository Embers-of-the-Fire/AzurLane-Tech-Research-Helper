/// Params:
///  - `$vbase`: base number.
///  - `$sinto`: the name of the struct to compare.
///  - `$tcp`: the value to compare.
///  - `$outo`: symbol change.
#[macro_export]
macro_rules! strict_check_larger {
    ($vbase: expr, $sinto: expr, $tcp: expr, $outo: ident) => {
        $vbase * match $sinto {
            Strict::None => 1.0,
            Strict::Strict(v) => if $tcp > v { $outo = true; 0.0 } else { 1.0 },
            Strict::Loose(v) => if $tcp > v { 10.0 } else { 1.0 },
        }
    };
}

/// Params:
///  - `$vbase`: base number.
///  - `$sinto`: the name of the struct to compare.
///  - `$tcp`: the value to compare.
///  - `$outo`: symbol change.
#[macro_export]
macro_rules! strict_check_smaller {
    ($vbase: expr, $sinto: expr, $tcp: expr, $outo: ident) => {
        $vbase * match $sinto {
            Strict::None => 1.0,
            Strict::Strict(v) => if $tcp < v { $outo = true; 0.0 } else { 1.0 },
            Strict::Loose(v) => if $tcp < v { 10.0 } else { 1.0 },
        }
    };
}