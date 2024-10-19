use ckks_engine::run_ckks_operations;
use ckks_engine::run_ckks_string_operations;

fn main() {
    ckks_engine::init_logging();
    // run_ckks_operations();
    run_ckks_string_operations();
}
