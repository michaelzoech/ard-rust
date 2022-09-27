macro_rules! assert_close {
    ($left:expr, $right:expr) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                use crate::math::CloseEq;
                if left_val.close_ne(right_val) {
                    panic!("assertion failed: `(left =~= right)`, (left: `{:?}`, right: `{:?}`)", left_val, right_val);
                }
            }
        }
    })
}

macro_rules! expect_eq {
    ($left:expr, $right:expr) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if left_val != right_val {
                    panic!("Expected {:?} to be {:?}, but was {:?}", stringify!($left), right_val, left_val);
                }
            }
        }
    })
}

macro_rules! expect_neq {
    ($left:expr, $right:expr) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if left_val == right_val {
                    panic!("Expected {:?} to be not {:?}, but was {:?}", stringify!($left), right_val, left_val);
                }
            }
        }
    })
}

macro_rules! expect_lt {
    ($left:expr, $right:expr) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if left_val >= right_val {
                    panic!("Expected {:?} ({:?}) to be less than {:?} ({:?})", stringify!($left), left_val, stringify!($right), right_val);
                }
            }
        }
    })
}
