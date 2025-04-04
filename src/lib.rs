#![allow(dead_code)]

use alpha_gen::AlphaGenerator;

// think of fractional part of root
pub fn root(nth: u16, rad: u32) {}

mod alpha_gen {
    // Let α be the next n digits of the radicand.
    pub struct AlphaGenerator {
        // operative number
        num: u32,
        // operative decimal places count
        plc: u32,
        // radix size
        siz: u32,
    }

    impl AlphaGenerator {
        pub fn new(num: u32, siz: u32) -> Self {
            if siz == 0 {
                panic!("0ᵗʰ root is strictly unsupported computation.");
                // that would mean seeking such root that is result of zero-time
                // applied division, that means root is argument but this would
                // be possible only for 1
            }

            let plc = if num == 0 {
                0
            } else {
                let places = num.ilog10() + 1;

                let full_blocks = places / siz;
                let fbs_size = full_blocks * siz;
                let divisible = fbs_size == places;

                match divisible {
                    | true => fbs_size,
                    | false => fbs_size + siz,
                }
            };

            Self { num, siz, plc }
        }

        pub fn next(&mut self) -> u32 {
            let num = self.num;

            if num == 0 {
                return 0;
            }

            let plc = self.plc - self.siz;

            let pow = 10u32.pow(plc);
            let alpha = num / pow;

            self.num = num % pow;
            self.plc = plc;

            alpha
        }
    }

    #[cfg(test)]
    mod tests_of_units {
        use crate::AlphaGenerator;

        #[test]
        fn basic_test() {
            let vals = [
                (1_234_567, 3, [1, 234, 567, 0, 0]),
                (11_2222_3333, 4, [11, 2222, 3333, 0, 0]),
            ];

            for v in vals {
                let mut generator = AlphaGenerator::new(v.0, v.1);

                for n in v.2 {
                    let next = generator.next();
                    assert_eq!(n, next);
                }
            }
        }

        #[test]
        fn greater_root_test() {
            let vals = [
                (123, 4, [123, 0, 0]),
                (123, 11, [123, 0, 0]),
                (12345_67890, 11, [12345_67890, 0, 0]),
            ];

            for v in vals {
                let mut generator = AlphaGenerator::new(v.0, v.1);

                for n in v.2 {
                    let next = generator.next();
                    assert_eq!(n, next);
                }
            }
        }

        #[test]
        fn divisible_by_root_test() {
            let number = 1234;
            let root = 2;

            let mut generator = AlphaGenerator::new(number, root);

            for n in [12, 34, 0, 0] {
                let next = generator.next();
                assert_eq!(n, next);
            }
        }

        #[test]
        fn zero_num_test() {
            let number = 0;
            let root = 1;

            let mut generator = AlphaGenerator::new(number, root);

            for _ in 0..3 {
                let next = generator.next();
                assert_eq!(0, next);
            }
        }

        #[test]
        #[should_panic(expected = "0ᵗʰ root is strictly unsupported computation.")]
        fn zero_root_test() {
            let number = u32::MAX;
            let root = 0;

            _ = AlphaGenerator::new(number, root);
        }
    }
}

// cargo test --release
