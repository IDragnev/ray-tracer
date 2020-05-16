use clap::{
    App,
    Arg,
};

pub struct Args {
    pub width: i32,
    pub height: i32,
    pub pixel_samples: i32,
    pub output: String,
    pub scene: String,
}

pub fn parse() -> Args {
    let matches = App::new("raytr")
        .author("Iliyan Dragnev")
        .about("A simple ray tracer")
        .arg(
            Arg::with_name("height")
                .short("h")
                .long("height")
                .value_name("Specifies the height of the produced image")
                .required(true)
                .takes_value(true)
                .default_value("300")
                .validator(&is_positive_integer_arg)
        )
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .value_name("Specifies the width of the produced image")
                .required(true)
                .takes_value(true)
                .default_value("300")
                .validator(&is_positive_integer_arg)
        )
        .arg(
            Arg::with_name("samples")
                .short("spp")
                .long("samples")
                .value_name("Specifies the number of samples for each pixel")
                .required(true)
                .takes_value(true)
                .default_value("32")
                .validator(&is_positive_integer_arg)
        )
        .arg(
            Arg::with_name("scene")
                .short("n")
                .long("scene")
                .value_name("Specifies the scene to be rendered")
                .required(true)
                .takes_value(true)
                .default_value("simple-light")
        )
        .arg(
            Arg::with_name("output")
                .value_name("Specifies the output file")
                .required(true)
                .index(1),
        )
        .get_matches();

    let height = matches.value_of("height")
        .unwrap()
        .parse()
        .unwrap();
    let width = matches.value_of("width")
        .unwrap()
        .parse()
        .unwrap();
    let pixel_samples = matches.value_of("samples")
        .unwrap()
        .parse()
        .unwrap();
    let output = matches.value_of("output")
        .unwrap()
        .to_owned();
    let scene = matches.value_of("scene")
        .unwrap()
        .to_owned();
    
    Args {
        width,
        height,
        pixel_samples,
        output,
        scene,
    }
}

fn is_positive_integer_arg(arg: String) -> Result<(), String> { 
    match arg.parse::<i32>() {
        Ok(i) if i > 0  => Ok(()),
        _               => Err("Positive interger expected".to_owned())
    }
}