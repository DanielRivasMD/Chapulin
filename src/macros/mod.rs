
#[macro_export]
macro_rules! effective_genome_length_calculator {
    ($gl: expr, $bs: expr, $bo: expr) => {
        effective_genome_length_calculator($gl, $bs, $bo)
    };
    ($gl: expr) => {
        effective_genome_length_calculator($gl, BIN_SIZE as f64, BIN_OVERLAP as f64)
    };
}

#[macro_export]
macro_rules! lambda_calculator {
    ($pr: expr, $egl: expr, $bs: expr) => {
        lambda_calculator($pr, $egl, $bs)
    };
    ($pr: expr, $egl: expr) => {
        lambda_calculator($pr, $egl, BIN_SIZE as f64)
    };
}
