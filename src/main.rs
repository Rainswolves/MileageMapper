use num_format::{Locale, ToFormattedString};
use rand::{Rng, thread_rng};
use mileage_mapper::{EntityType, parse_args};

struct OscillatingTable<'f> {
    input: &'f mut u64,
    step: u64,
    bound: RNGBound,
    pattern: Vec<u16>
}

struct RNGBound {
    lo_bound: u8,
    hi_bound: u8,
}

impl Default for RNGBound {
    fn default() -> Self {
        Self { lo_bound: 7, hi_bound: 12 }
    }
}

fn linear(input: &mut u64, amount: u16, step: u64) -> Vec<u64> {
    let mut linear_vals: Vec<u64> = Vec::new();

    for _ in 0..amount {
        linear_vals.push(*input);
        *input += step;
        linear_vals.push(*input);
    }

    return linear_vals
}

fn oscillating(d: OscillatingTable) -> Vec<u64> {
    let mut oscil_vals: Vec<u64> = Vec::new();

    for s in d.pattern {
        for x in linear(d.input, s, d.step) {
            oscil_vals.push(x);
        }

        let rnd_amount = thread_rng().gen_range(d.bound.lo_bound..d.bound.hi_bound);
        *d.input += u64::from(rnd_amount);
    }

    return oscil_vals
}

fn en_format(target: u64) -> String {
    return target.to_formatted_string(&Locale::en)
}

fn main() {

    let args = parse_args();

    match args.entity_type {
        EntityType::Osc(mut osc_values) => {
            let mut b = RNGBound::default();
            if osc_values.lo_offset.is_some() {
                b = RNGBound {
                    hi_bound: osc_values.hi_offset.unwrap(),
                    lo_bound: osc_values.lo_offset.unwrap(),
                }
            }
            let data = OscillatingTable {
                input: &mut osc_values.start_amount,
                step: osc_values.increment,
                bound: b,
                pattern: osc_values.pattern    
            };

            for pair in oscillating(data).chunks_exact(2) {
                println!("{} - {}", en_format(pair[0]), en_format(pair[1]));
            }
        }
        EntityType::Lin(mut linear_amount) => {
            for pair in linear(&mut linear_amount.start_amount, linear_amount.amount, linear_amount.increment).chunks_exact(2) {
                println!("{} - {}", en_format(pair[0]), en_format(pair[1]));
            }
        }
    }
}
