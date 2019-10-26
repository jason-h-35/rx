use rx;
use rx::session;

use pico_args;

use std::process;

const HELP: &'static str = r#"
A Modern & Extensible Pixel Editor
Alexis Sellier <self@cloudhead.io>

USAGE
    rx [OPTIONS] [<path>..]

OPTIONS
    -h, --help           Prints help
    -V, --version        Prints version

    --verbosity <level>  Set verbosity level (0-5)
    --record <file>      Record user input to a file
    --replay <file>      Replay user input from a file
    --width <width>      Set the window width
    --height <height>    Set the window height
"#;

fn main() {
    if let Err(e) = self::execute(pico_args::Arguments::from_env()) {
        eprintln!("rx: error: {}", e.as_ref());
        process::exit(1);
    }
}

fn execute(
    mut args: pico_args::Arguments,
) -> Result<(), Box<dyn std::error::Error>> {
    rx::ALLOCATOR.reset();

    let default = rx::Options::default();

    if args.contains(["-h", "--help"]) {
        println!("rx v{}{}", rx::VERSION, HELP);
        return Ok(());
    }

    if args.contains(["-V", "--version"]) {
        println!("rx v{}", rx::VERSION);
        return Ok(());
    }

    if args.contains("--replay") && args.contains("--record") {
        return Err("'--replay' and '--record' can't both be specified".into());
    }

    let log = match args.opt_value_from_str("--verbosity")?.unwrap_or(0) {
        0 => "rx=warn",
        1 => "rx=info,error",
        2 => "rx=debug,error",
        3 => "rx=debug,error",
        4 => "rx=debug,info",
        _ => "debug",
    };

    let exec =
        if let Some(path) = args.opt_value_from_str::<_, String>("--replay")? {
            session::ExecutionMode::replaying(path)?
        } else if let Some(path) =
            args.opt_value_from_str::<_, String>("--record")?
        {
            session::ExecutionMode::recording(path)?
        } else {
            default.exec
        };

    let options = rx::Options {
        exec,
        log,
        width: args.opt_value_from_str("--width")?.unwrap_or(default.width),
        height: args
            .opt_value_from_str("--height")?
            .unwrap_or(default.height),
    };

    let paths = args.free()?;

    rx::init(&paths, options).map_err(|e| e.into())
}
