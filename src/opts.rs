use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Opts {
    /// Set the density of the blocks, the value should be between 0 and 1
    #[structopt(short = "d", long = "block-density", default_value = "0.2")]
    pub block_density: f64,
    /// Set the strength of the blocks
    #[structopt(short = "s", long = "block-strength", default_value = "1")]
    pub block_strength: u16,
    /// Set the width of the bar
    #[structopt(short = "w", long = "bar-width", default_value = "10")]
    pub bar_width: u16,
    /// Set the power of the ball
    #[structopt(short = "p", long = "ball-power", default_value = "1")]
    pub ball_power: u16,
}
