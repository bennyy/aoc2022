pub trait AdventOfCode {
    fn day_str(&self) -> String;

    fn run_puzzle1(&mut self, input_str: String);
    fn run_puzzle2(&mut self, input_str: String);

    fn get_puzzle1_result(&self) -> i32;
    fn get_puzzle2_result(&self) -> i32;
}

#[macro_export]
macro_rules! _not_implemented {
    ($type: ty, $type2: ident, $day_str:expr) => {
        impl $type {
            // "tt" can't construct any structs. Need to be a ident.
            pub fn new() -> $type {
                $type2 {}
            }
        }

        impl AdventOfCode for $type {
            fn day_str(&self) -> String {
                $day_str.to_owned()
            }

            fn run_puzzle1(&mut self, _input_str: String) {}

            fn run_puzzle2(&mut self, _input_str: String) {}

            fn get_puzzle1_result(&self) -> i32 {
                0
            }

            fn get_puzzle2_result(&self) -> i32 {
                0
            }
        }
    };
}

#[macro_export]
macro_rules! not_implemented {
    ($type: tt, $day_str:expr) => {
        // Split the type into two, one for ty (type) and one for ident.
        $crate::_not_implemented!($type, $type, $day_str);
    };
}

#[macro_export]
macro_rules! puzzle1_test {
    ($($type:ident),+ $(,)?, $test_expected:expr, $expected:expr) => ($(
        use $crate::{file_util};

        #[test]
        fn puzzle_1() {
            let mut day = $type::new();

            let test_str: String = file_util::file_to_string(format!("inputs/{}_test.txt", day.day_str()));
            day.run_puzzle1(test_str);
            assert_eq!($test_expected, day.get_puzzle1_result());

            let main_str: String = file_util::file_to_string(format!("inputs/{}.txt", day.day_str()));
            day.run_puzzle1(main_str);
            assert_eq!($expected, day.get_puzzle1_result());
        }
    )+)
}

#[macro_export]
macro_rules! puzzle2_test {
    ($($type:ident),+ $(,)?, $test_expected:expr, $expected:expr) => ($(
        #[test]
        fn puzzle_2() {
            let mut day = $type::new();

            let test_str: String = file_util::file_to_string(format!("inputs/{}_test.txt", day.day_str()));
            day.run_puzzle2(test_str);
            assert_eq!($test_expected, day.get_puzzle2_result());

            let main_str: String = file_util::file_to_string(format!("inputs/{}.txt", day.day_str()));
            day.run_puzzle2(main_str);
            assert_eq!($expected, day.get_puzzle2_result());
        }
    )+)
}
