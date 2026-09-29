use std::{time::{SystemTime, UNIX_EPOCH, Duration}, thread};

pub fn pseudo_randf64(min:f64, max:f64, microsecond_delay:u8) -> f64 {
    let num_zero_to_ten_thousand = (
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos()%10000
    ) as f64;
    let old_max = 10000.;
    let old_min = 0.;
    let old_range = (old_max - old_min);
    let new_range = (max - min);
    let rescaled_value = ( (num_zero_to_ten_thousand - old_min) / (old_range) ) * (new_range) + min;
    thread::sleep(Duration::from_micros(microsecond_delay as u64)); // to allow direct re-uses
    rescaled_value
}