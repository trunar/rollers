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

    /// Show the average instead of rolling
    #[arg(short, long)]
    average: bool,

    /// Keep only the highest N dice
    #[arg(long, value_name = "N")]
    highest: Option<usize>,

    /// Keep only the lowest N dice
    #[arg(long, value_name = "N")]
    lowest: Option<usize>,
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
        perform_roll(count, &die_type, modifier, &args);
    }
}

fn perform_roll(count: u32, die: &DieSides, modifier: i32, args: &Args) {
    let mut rng = rand::thread_rng();
    let mut rolls = Vec::new();

    for _ in 0..count {
        let val = match die {
            DieSides::Standard(s) => rng.gen_range(1..=*s) as i32,
            DieSides::Fudge => rng.gen_range(-1..=1),
        };
        rolls.push(val);
    }

    // Keep track of original dice pool and kept dice
    let pool = rolls.clone();
    let mut kept = rolls.clone();

    // Apply Highest/Lowest logic
    if let Some(n) = args.highest {
        kept.sort_by(|a, b| b.cmp(a)); // Sort descending
        kept.truncate(n);
    } else if let Some(n) = args.lowest {
        kept.sort(); // Sort ascending
        kept.truncate(n);
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