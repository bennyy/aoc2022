pub trait AdventOfCode {
    fn day_str(&self) -> String;

    fn run_puzzle1(&mut self, file_path: String);
    fn run_puzzle2(&mut self, file_path: String);

    fn get_puzzle1_result(&self) -> i32;
    fn get_puzzle2_result(&self) -> i32;
}

#[macro_export]
macro_rules! not_implemented {
    ($($t:ty),+ $(,)?, $day_str:expr) => ($(
        impl AdventOfCode for $t {
            fn day_str(&self) -> String {
                $day_str.to_owned()
            }

            fn run_puzzle1(&mut self, _file_path: String) {
            }

            fn run_puzzle2(&mut self, _file_path: String) {
            }

            fn get_puzzle1_result(&self) -> i32 {
                0
            }

            fn get_puzzle2_result(&self) -> i32 {
                0
            }

        }
    )+)
}
