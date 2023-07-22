use num_format::{Locale, ToFormattedString};
use rand::{Rng, thread_rng};
use mileage_mapper::{EntityType, parse_args};

fn linear(input: &mut u64, amount: u16, step: u64) -> Vec<u64> {
    let mut linear_vals: Vec<u64> = Vec::new();

    for _ in 0..amount {
        linear_vals.push(*input);
        *input += step;
        linear_vals.push(*input);
    }

    return linear_vals
}

fn oscillating(input: &mut u64, step: u64, lo_bound: u8, hi_bound: u8, pattern: Vec<u16>) -> Vec<u64> {
    let mut oscil_vals: Vec<u64> = Vec::new();

    for s in pattern {
        for x in linear(input, s, step) {
            oscil_vals.push(x);
        }
        let rnd_amount = thread_rng().gen_range(lo_bound..hi_bound);
        *input += u64::from(rnd_amount);
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
            for pair in oscillating(&mut osc_values.start_amount, osc_values.increment, osc_values.lo_offset, osc_values.hi_offset, osc_values.pattern).chunks_exact(2) {
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
