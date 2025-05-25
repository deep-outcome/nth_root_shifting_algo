// Variables and their meaning
// --------------------------------------------------
// B  — base of number system, e.g. binary or ternary
// n  — root/radix degree
// x  — radicand
// y  — root/radix
// r  — remainder
// α  — next n places of radicand
// β  — root next number
// y' — new y for next iteration
// r' — new r for next iteration

use alpha_gen::AlphaGenerator;

const BASE: u32 = 10;

/// `nth` – radix degree
/// `rad` – radicand
// n, x
pub fn root(nth: u8, rad: u32) -> Option<u32> {
    if nth == 0 {
        return None;
    }

    let nth = nth as u32;

    // root/radix
    // y
    let mut rax = 0;
    // remainder
    // r
    let mut rem = 0;

    // decadic base powered by degree
    // base degree power
    // Bⁿ
    let bdp = BASE.pow(nth);

    // n -1
    let nth_less = nth - 1;

    // degree base degree less power
    // nBⁿ⁻¹
    let dbdlp = nth * BASE.pow(nth_less);

    let mut agen = AlphaGenerator::new(rad, nth);

    // integer root, otherwise some kind (degree) of precision must be used
    loop {
        // α
        let alpha = agen.next();
        // operatives
        // y', r'
        let (orax, orem) = step::next(rax, rem, bdp, alpha, nth, nth_less, dbdlp);

        let orax_pow = orax.pow(nth);

        if orax_pow > rad {
            break;
        }

        rax = orax;

        if orax_pow == rad {
            break;
        }

        rem = orem;
    }

    Some(rax)
}

#[cfg(test)]
mod tests_of_units {
    use crate::root;

    #[test]
    fn basic_test() {
        assert_eq!(Some(2), root(3, 8));
    }

    #[test]
    fn zero_root_test() {
        assert_eq!(None, root(0, u32::MAX));
    }

    #[test]
    fn first_root() {
        let vals = [0, 1, 2, 3, 10, 100, 999, 1_000_000, 9_999_999];

        for &v in vals.iter() {
            assert_eq!(Some(v), root(1, v), "val: {v}");
        }
    }

    #[test]
    fn sqrt_basic_test() {
        #[rustfmt::skip]
        let vals = [
            (0, [0].as_slice()),
            (1, [1,3].as_slice()),
            (2, [4,8].as_slice()),
            (3, [9,15].as_slice()),
            (4, [16,24].as_slice()),
            (5, [25,35].as_slice())];

        for v in vals.iter() {
            for &n in v.1 {
                assert_eq!(Some(v.0), root(2, n), "exp: {}, inp: {}", v.0, n);
            }
        }
    }

    #[test]
    fn cbrt_basic_test() {
        #[rustfmt::skip]
        let vals = [
            (0,[0].as_slice()),
            (1,[1,7].as_slice()), 
            (2,[8,26].as_slice()),
            (3,[27,63].as_slice()),
            (4,[64,124].as_slice()),
            (5,[125,215].as_slice())];

        for v in vals.iter() {
            for &n in v.1 {
                assert_eq!(Some(v.0), root(3, n), "exp: {}, inp: {}", v.0, n);
            }
        }
    }

    #[test]
    fn integer_root_test() {
        #[rustfmt::skip]
        let vals = [
            (4, 4, 256),
            (7, 5, 16_807),
            (100, 4, 1_00_00_00_00),
            (217, 3, 10_218_313),
            (5560, 2, 30_913_600),
            (1222, 3, 1_824_793_048), 
            (177, 4, 981_506_241),
            (793, 3, 498_677_257),
            (313, 3, 30_664_297),
            // works only in release
            (4, 14, 268_435_456),            
            (2, 30, 1_073_741_824),
            // ill-fated overflows                                    
            // (2, 31, 2147483648), 
            // (4, 15, 1073741824),
        ];
        for v in vals {
            assert_eq!(
                Some(v.0),
                root(v.1, v.2),
                "exp: {}, deg: {}, inp: {}",
                v.0,
                v.1,
                v.2
            );
        }
    }

    #[test]
    fn rounded_root_test() {
        #[rustfmt::skip]
        let vals = [
            (17, 2, 312),               // ≈ 17.7
            (9, 4, 9999),               // ≈ 9.9998
            (9, 3, 999),                // ≈ 9.997
            (9, 2, 99),                 // ≈ 9.95
            (99, 2, 9999),              // ≈ 99.995
            (21, 3, 9999),              // ≈ 21.5            
            (20, 4, 173_479),           // ≈ 20.41
            
            // works only in release
            // does not work with guess
            // (2, 17, 16_777_215),        // ≈ 2.661            
            (3, 13, 33_554_431),        // ≈ 3.79            
            (31629, 2, 1_000_400_400),  // ≈ 31629.11
            (45, 5, 200_300_010),       // ≈ 45.7            
            
            // ill-fated overflows
            // (5, 12, 900_900_009),    // ≈ 5.575
            // (2, 26, 90_900_009),     // ≈ 2.02                                     
        ];
        for v in vals {
            assert_eq!(
                Some(v.0),
                root(v.1, v.2),
                "exp: {}, deg: {}, inp: {}",
                v.0,
                v.1,
                v.2
            );
        }
    }
}

mod step {

    // β is largest number complying formula
    // (By +β)ⁿ -Bⁿyⁿ ≤ Bⁿr +α
    pub const fn next(
        rax: u32,         // y
        rem: u32,         // r
        bdp: u32,         // Bⁿ
        alpha: u32,       // α
        degree: u32,      // n
        degree_less: u32, // n -1
        dbdlp: u32,       // nBⁿ⁻¹
    ) -> (u32, u32) {
        next_actual(
            rax,
            rem,
            bdp,
            alpha,
            degree,
            degree_less,
            dbdlp,
            #[cfg(test)]
            &mut 0,
            #[cfg(test)]
            &mut 0,
            #[cfg(test)]
            &mut 0,
            #[cfg(test)]
            &mut 0,
            #[cfg(test)]
            &mut 0,
            #[cfg(test)]
            &mut 0,
            #[cfg(test)]
            &mut None,
        )
    }

    const fn next_actual(
        mut rax: u32,     // y
        rem: u32,         // r
        bdp: u32,         // Bⁿ
        alpha: u32,       // α
        degree: u32,      // n
        degree_less: u32, // n -1
        dbdlp: u32,       // nBⁿ⁻¹
        #[cfg(test)] wrax_out: *mut u32,
        #[cfg(test)] rax_pow_less_out: *mut u32,
        #[cfg(test)] sub_out: *mut u32,
        #[cfg(test)] lim_out: *mut u32,
        #[cfg(test)] div_out: *mut u32,
        #[cfg(test)] beta_out: *mut u32,
        #[cfg(test)] guess_out: *mut Option<u32>,
    ) -> (u32, u32) {
        // By, widen rax
        let wrax = rax * super::BASE;

        // yⁿ⁻¹
        let rax_pow_less = rax.pow(degree_less);

        // Bⁿyⁿ, subtrahend
        let sub = bdp * (rax_pow_less * rax);
        // Bⁿr +α, limit
        let lim = bdp * rem + alpha;

        // y' =By +β, β =0
        rax = wrax;

        // (By +β)ⁿ -Bⁿyⁿ
        // β =0 =>(By)ⁿ -Bⁿyⁿ =0
        let mut max = 0;

        // let make initial guess, if possible
        let (mut guess, mut beta) = {
            let mut g = 0;

            if rax_pow_less > 0 {
                let div = dbdlp * rax_pow_less;

                #[cfg(test)]
                unsafe {
                    *div_out = div;
                }

                // Bⁿr +α ÷(nBⁿ⁻¹ ·yⁿ⁻¹)
                g = lim / div;
            }

            if g > 0 {
                (Some(g), g)
            } else {
                (None, 1)
            }
        };

        #[cfg(test)]
        unsafe {
            *wrax_out = wrax;
            *rax_pow_less_out = rax_pow_less;
            *sub_out = sub;
            *lim_out = lim;
            *beta_out = beta;
            *guess_out = guess;
        }

        // seeking largest beta that
        // (By +β)ⁿ -Bⁿyⁿ ≤ Bⁿr +α
        'incr: loop {
            // o stands for operative

            // y' =By +β
            let orax = wrax + beta;
            // (By +β)ⁿ
            let orax_deg_pow = orax.pow(degree);
            // (By +β)ⁿ -Bⁿyⁿ
            let omax = orax_deg_pow - sub;

            // (By +β)ⁿ -Bⁿyⁿ ≤ Bⁿr +α
            if omax > lim {
                if let Some(g) = guess {
                    if g == beta {
                        break 'incr;
                    }

                    guess = None;
                }

                break 'incr;
            }

            rax = orax;
            max = omax;

            // (By +β)ⁿ -Bⁿyⁿ ≤ Bⁿr +α
            if omax == lim {
                if guess.is_some() {
                    guess = None;
                }
                break 'incr;
            }

            beta += 1;
        }

        if guess.is_none() {
            // r' =Bⁿr +α -((By +β)ⁿ -Bⁿyⁿ)
            return (rax, lim - max);
        }

        // seeking largest beta that
        // (By +β)ⁿ -Bⁿyⁿ ≤ Bⁿr +α
        beta -= 1;
        'decr: loop {
            // o stands for operative

            // y' =By +β
            let orax = wrax + beta;
            // (By +β)ⁿ
            let orax_deg_pow = orax.pow(degree);
            // (By +β)ⁿ -Bⁿyⁿ
            let omax = orax_deg_pow - sub;

            // (By +β)ⁿ -Bⁿyⁿ ≤ Bⁿr +α
            if omax <= lim {
                rax = orax;
                max = omax;

                break 'decr;
            }

            beta -= 1;
        }

        // r' =Bⁿr +α -((By +β)ⁿ -Bⁿyⁿ)
        (rax, lim - max)
    }

    #[cfg(test)]
    mod tets_of_units {

        mod next_actual {
            use crate::step::next_actual;

            #[test]
            fn basic_test() {
                let rax = 3;
                let rem = 15;
                let bdp = 1000;
                let alpha = 133;
                let degree = 3;
                let degree_less = 2;
                let dbdlp = 300;

                let mut wrax_out = 0;
                let mut rax_pow_less_out = 0;
                let mut sub_out = 0;
                let mut lim_out = 0;
                let mut div_out = 0;
                let mut beta_out = 0;
                let mut guess_out = None;

                _ = next_actual(
                    rax, rem, bdp, alpha, degree, degree_less, dbdlp, &mut wrax_out,
                    &mut rax_pow_less_out, &mut sub_out, &mut lim_out, &mut div_out, &mut beta_out,
                    &mut guess_out,
                );

                assert_eq!(30, wrax_out);
                assert_eq!(9, rax_pow_less_out);
                assert_eq!(27_000, sub_out);
                assert_eq!(15_133, lim_out);
                assert_eq!(2_700, div_out);

                let beta = 15_133 / 2_700;
                assert_eq!(beta, beta_out);
                assert_eq!(Some(beta), guess_out);
            }
        }
    }
}

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

// cargo fmt && cargo test --release
