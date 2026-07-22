use logging::Frog;

fn main() {
    env_logger_successor::init();

    let mut skippy = Frog::new();
    skippy.hop();
    skippy.hop();
    skippy.hop();
    skippy.hop();
    skippy.hop();
    skippy.sleep();
    skippy.sleep();

    // Challenge: Go back to lib.rs and set the `target: ` argument for each logging call to be the
    // path to the function.  For example, `Frog::new`
}
