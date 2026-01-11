use clap::Parser;
use rand::Rng;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(author, about, version)]
struct Args {
    /// Dice notation (e.g., 2d6, 4dF, 1d20+5)
    input: String,

    /// Only show the final result
    #[arg(short, long)]
    quiet: bool,

    /// Roll again on max value
    #[arg(short, long, conflicts_with_all = ["average"])]
    exploding: bool,

    /// Repeat the roll N times
    #[arg(short, long, value_name = "N", default_value_t = 1, conflicts_with_all = ["average"])]
    repeat: u32,

    /// Show the average instead of rolling
    #[arg(short, long, conflicts_with_all = ["highest", "lowest", "drop_highest", "drop_lowest", "exploding", "repeat"])]
    average: bool,

    /// Keep only the highest N dice
    #[arg(long, value_name = "N", conflicts_with_all = ["average", "lowest", "drop_highest", "drop_lowest"])]
    highest: Option<usize>,

    /// Keep only the lowest N dice
    #[arg(long, value_name = "N", conflicts_with_all = ["average", "highest", "drop_highest", "drop_lowest"])]
    lowest: Option<usize>,

    /// Drop the highest N dice
    #[arg(long, value_name = "N", conflicts_with_all = ["average", "highest", "lowest", "drop_lowest"])]
    drop_highest: Option<usize>,

    /// Drop the lowest N dice
    #[arg(long, value_name = "N", conflicts_with_all = ["average", "highest", "lowest", "drop_highest"])]
    drop_lowest: Option<usize>,
}

#[derive(Debug)]
enum DieSides {
    Standard(u32),
    Fudge,
}

fn main() {
    let args = Args::parse();
    
    // Regex: captures [count]d[sides/F][modifier]
    let re = Regex::new(r"(?P<count>\d+)d(?P<sides>\d+|F)(?P<mod>[+-]\d+)?").unwrap();

    let caps = re.captures(&args.input).expect("Invalid dice format! Use XdX or XdF");

    let count: u32 = caps["count"].parse().unwrap_or(1);
    let modifier: i32 = caps.name("mod").map_or(0, |m| m.as_str().parse().unwrap());
    
    let die_type = match &caps["sides"] {
        "F" => DieSides::Fudge,
        s => DieSides::Standard(s.parse().unwrap()),
    };

    if args.average {
        calculate_average(count, &die_type, modifier, &args);
    } else {
        for i in 0..args.repeat {
            if !args.quiet && args.repeat > 1 {
                println!("\n--- Roll {} ---", i + 1);
            }
            perform_roll(count, &die_type, modifier, &args);
        }
    }
}

fn perform_roll(count: u32, die: &DieSides, modifier: i32, args: &Args) {
    let mut rng = rand::thread_rng();
    let mut rolls = Vec::new();

    for _ in 0..count {
        match die {
            DieSides::Standard(s) => {
                let mut roll = rng.gen_range(1..=*s) as i32;
                rolls.push(roll);
                
                // Exploding logic
                if args.exploding && *s > 1 {
                    while roll == *s as i32 {
                        roll = rng.gen_range(1..=*s) as i32;
                        rolls.push(roll);
                    }
                }
            }
            DieSides::Fudge => {
                let mut roll = rng.gen_range(-1..=1);
                rolls.push(roll);
                
                // Fudge explosion
                if args.exploding {
                    while roll == 1 {
                        roll = rng.gen_range(-1..=1);
                        rolls.push(roll);
                    }
                }
            }
        };
    }

    // Keep track of original dice pool and kept dice
    let mut pool = rolls.clone();
    let mut kept = rolls.clone();

    let final_count = rolls.len();

    // Determine the target keep count and sorting strategy
    let (keep_count, keep_highest) = match (args.highest, args.lowest, args.drop_highest, args.drop_lowest) {
        (Some(n), _, _, _) => (n, true),
        (_, Some(n), _, _) => (n, false),
        (_, _, Some(n), _) => (final_count.saturating_sub(n), false),
        (_, _, _, Some(n)) => (final_count.saturating_sub(n), true),
        _ => (final_count, true),
    };

    // Sort and truncate
    if keep_count < final_count {
        if keep_highest {
            pool.sort_unstable_by(|a, b| b.cmp(a));
            kept.sort_unstable_by(|a, b| b.cmp(a));
        } else {
            pool.sort_unstable();
            kept.sort_unstable();
        }
        kept.truncate(keep_count);
    }

    let total: i32 = kept.iter().sum::<i32>() + modifier;

    if args.quiet {
        println!("{}", total);
    } else {
        let roll_strs: Vec<String> = pool.iter().map(|&r| format_die(r, die)).collect();
        let kept_strs: Vec<String> = kept.iter().map(|&r| format_die(r, die)).collect();

        println!();
        println!("  {:<10} {}", "Pool:", roll_strs.join(", "));
        
        // Only show "Kept" if we actually filtered the dice
        if kept.len() < pool.len() {
            println!("  {:<10} {}", "Kept:", kept_strs.join(", "));
        }

        if modifier != 0 {
            println!("  {:<10} {}", "Modifier:", if modifier >= 0 { format!("+{}", modifier) } else { modifier.to_string() });
        }

        println!("  {:<10} \x1b[1m{}\x1b[0m", "Total:", total);
    }
}

fn format_die(val: i32, die: &DieSides) -> String {
    if matches!(die, DieSides::Fudge) {
        match val {
            1 => "+".to_string(),
            -1 => "-".to_string(),
            _ => "0".to_string(),
        }
    } else {
        val.to_string()
    }
}

fn calculate_average(count: u32, die: &DieSides, modifier: i32, args: &Args) {
    let avg_per_die = match die {
        DieSides::Standard(s) => (*s as f32 + 1.0) / 2.0,
        DieSides::Fudge => 0.0,
    };

    let total_avg = (count as f32 * avg_per_die) + modifier as f32;

    if args.quiet {
        println!("{}", total_avg);
    } else {
        println!("\n  Average: {:.2}", total_avg);
    }
}