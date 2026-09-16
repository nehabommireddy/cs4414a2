use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::ExitCode;
use std::time::Instant;

mod corpus;
mod hamming;

enum Mode {
    Naive,
    Blocked,
    Packed,
    Swar,
    Fast,
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    // Check for arguments
    if args.len() < 3 || args.len() > 4 {
        eprintln!("Usage: p2_edit dictionary mode [results]");
        eprintln!("Mode options: naive, blocked, packed, swar, fast");
        return ExitCode::FAILURE;
    }

    // Check correctness of mode flag
    let mode = if &args[2] == "naive" {
        Mode::Naive
    } else if &args[2] == "blocked" {
        Mode::Blocked
    } else if &args[2] == "packed" {
        Mode::Packed
    } else if &args[2] == "swar" {
        Mode::Swar
    } else if &args[2] == "fast" {
        Mode::Fast
    } else {
        eprintln!("Unknown mode {}", args[2]);
        eprintln!("Valid modes: naive, blocked, packed, swar, fast");
        return ExitCode::FAILURE;
    };

    // Load dictionary
    let start = Instant::now();
    let path = Path::new(&args[1]);
    let dict = match corpus::load_words(path) {
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
        Ok(d) => d,
    };
    println!("# Load time is {:?}", start.elapsed());

    let start = Instant::now();
    let result = match mode {
        Mode::Naive => hamming::basic_str::mean_dists_dict(&dict),
        Mode::Blocked => hamming::block_str::mean_dists_dict(&dict),
        Mode::Packed => hamming::basic_word::mean_dists_dict(&dict),
        Mode::Swar => hamming::swar_word::mean_dists_dict(&dict),
        Mode::Fast => hamming::optimized::mean_dists_dict(&dict),
    };
    println!("# Total computation time is {:?}", start.elapsed());

    if args.len() == 4 {
        let mut f = match File::create(&args[3]) {
            Err(e) => {
                eprintln!("{e}");
                return ExitCode::FAILURE;
            }
            Ok(f) => f,
        };
        for (word, dist) in dict.iter().zip(result.iter()) {
            if let Err(e) = writeln!(&mut f, "{word},{dist}") {
                eprintln!("{e}");
                return ExitCode::FAILURE;
            }
        }
    }

    ExitCode::SUCCESS
}
