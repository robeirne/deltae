use deltae::*;
use std::error::Error;

mod cli;

fn main() -> Result<(), Box<dyn Error>> {
    //Parse command line arguments with clap
    let matches = cli::app().get_matches();

    let method: DEMethod = matches.value_of("METHOD").expect("No default METHOD").parse()?;
    let color_type = matches.value_of("COLORTYPE").expect("No default COLORTYPE");
    let color0 = matches.value_of("COLOR0").expect("COLOR0 required");
    let color1 = matches.value_of("COLOR1").expect("COLOR1 required");

    let delta = match color_type {
        "lab" => color0.parse::<CieLabValue>()?.delta(color1.parse::<CieLabValue>()?, method),
        "lch" => color0.parse::<LchValue>()?.delta(color1.parse::<LchValue>()?, method),
        "xyz" => color0.parse::<CieXyzValue>()?.delta(color1.parse::<CieXyzValue>()?, method),
        _ => unreachable!("COLORTYPE: `{}`", color_type),
    };

    println!("{}", delta);

    Ok(())
}
