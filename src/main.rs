use clap::Parser;
use std::{
    thread,
    time::{self, Duration},
};

/// A simple CLI timer with an ASCII banner.
#[derive(Parser)]
#[command(author = "Your Name", version = "1.0", about = "A CLI timer with an ASCII banner", long_about = None)]
struct Cli {
    /// The duration in seconds
    #[arg(short, long, default_value = "0")]
    seconds: u64,

    /// The duration in minutes
    #[arg(short, long, default_value = "0")]
    minutes: u64,

    /// The duration in hours
    #[arg(short, long, default_value = "0")]
    ours: u64,
}

fn main() {
    let args = Cli::parse();

    if args.ours <= 23 && args.minutes / 60 <= 23 && (args.seconds / 60) / 60 <= 23 {
        let total_seconds = args.seconds + args.minutes * 60 + args.ours * 3600;

        if total_seconds == 0 {
            println!("Please specify a duration greater than 0.");
            return;
        }

        for remaining in (0..=total_seconds).rev() {
            let hours = remaining / 3600;
            let minutes = (remaining % 3600) / 60;
            let seconds = remaining % 60;

            clear_screen();
            display_banner(hours, minutes, seconds);

            if remaining > 0 {
                thread::sleep(time::Duration::from_secs(1));
            }
        }

        println!("Time's up!");
    }

    println!("days are not yet implemented")
}

fn display_banner(hours: u64, minutes: u64, seconds: u64) {
    let time_string = format!("{:}:{:}:{:}", hours, minutes, seconds);
    let ascii_digits = get_ascii_digits(&time_string);
    for line in 0..5 {
        for ch in &ascii_digits {
            print!("{} ", ch[line]);
        }
        println!();
    }
}

fn get_ascii_digits(time_string: &str) -> Vec<[&'static str; 5]> {
    let digits = [
        ["  ___  ", " / _ \\ ", "| | | |", "| |_| |", " \\___/ "], // 0
        [" _ ", "/ |", "| |", "| |", "|_|"],                       // 1
        [" ____  ", "|___ \\ ", "  __) |", " / __/ ", "|_____|"],  // 2
        [" _____ ", "|___ / ", "  |_ \\ ", " ___) |", "|____/ "],  // 3
        [" _  _   ", "| || |  ", "| || |_ ", "|__   _|", "   |_|  "], // 4
        [" ____  ", "| ___| ", "|___ \\ ", " ___) |", "|____/ "],  // 5
        ["  __   ", " / /_  ", "| '_ \\ ", "| (_) |", " \\___/ "], // 6
        [" _____ ", "|___  |", "   / / ", "  / /  ", " /_/   "],   // 7
        ["  ___  ", " ( _ ) ", " / _ \\ ", "| (_) |", " \\___/ "], // 8
        ["  ___  ", " / _ \\ ", "| (_) |", " \\__, |", "   /_/ "], // 9
        [" _ ", "(_)", "   ", " _ ", "(_)"],                       // :
    ];

    time_string
        .chars()
        .map(|c| match c {
            '0' => digits[0],
            '1' => digits[1],
            '2' => digits[2],
            '3' => digits[3],
            '4' => digits[4],
            '5' => digits[5],
            '6' => digits[6],
            '7' => digits[7],
            '8' => digits[8],
            '9' => digits[9],
            ':' => digits[10],
            _ => ["      ", "      ", "      ", "      ", "      "],
        })
        .collect()
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}
