
#[macro_export]
macro_rules! effective_genome_length_calculator {
    ($gl: expr, $bs: expr, $bo: expr) => {
        effective_genome_length_calculator($gl, $bs, $bo)
    };
    ($gl: expr) => {
        effective_genome_length_calculator($gl, BIN_SIZE as f64, BIN_OVERLAP as f64)
    };
}

