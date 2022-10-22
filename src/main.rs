use rand::{thread_rng, Rng};

struct Nickname {
    adjective: String,
    prefix: String,
    noun: String,
}

impl Nickname {
    pub fn new(adjective: &str, prefix: &str, noun: &str) -> Self {
        Self {
            adjective: String::from(adjective),
            prefix: String::from(prefix),
            noun: String::from(noun),
        }
    }
}

fn main() {
    let mut rng = thread_rng();

    let nicknames = [
        Nickname::new("agent", "", ""),
        Nickname::new("bädäss", "bädäss", "bädäss"),
        Nickname::new("", "bären", "bär"),
        Nickname::new("dickes", "", ""),
        Nickname::new("dummbatziges", "dummbatz", "dummbatz"),
        Nickname::new("freches", "", ""),
        Nickname::new("general", "", ""),
        Nickname::new("grosses", "gross", ""),
        Nickname::new("gutes", "", ""),
        Nickname::new("härziges", "", ""),
        Nickname::new("huere", "", ""),
        Nickname::new("", "hunde", "hund"),
        Nickname::new("", "", "hundeli"),
        Nickname::new("", "", "iisbär"),
        Nickname::new("kleines", "", ""),
        Nickname::new("kluges", "", ""),
        Nickname::new("", "kuschel", "kuscheli"),
        Nickname::new("", "lieblings", ""),
        Nickname::new("madame", "", "madame"),
        Nickname::new("", "mause", "maus"),
        Nickname::new("", "mäuse", "mäuschen"),
        Nickname::new("", "mausi", "mausi"),
        Nickname::new("", "", "meite"),
        Nickname::new("", "", "meiteli"),
        Nickname::new("", "meiti", "meiti"),
        Nickname::new("missus", "", ""),
        Nickname::new("", "monster", "monster"),
        Nickname::new("", "näsi", "näsi"),
        Nickname::new("", "nöff", "nöff"),
        Nickname::new("", "puffel", "puffel"),
        Nickname::new("", "puff", "puff"),
        Nickname::new("", "", "pupsi"),
        Nickname::new("", "quietsch", ""),
        Nickname::new("richtiges", "", ""),
        Nickname::new("riktastisches", "rika", "rika"),
        Nickname::new("", "", "schatzi"),
        Nickname::new("", "schatz", "schatz"),
        Nickname::new("", "schnapp", ""),
        Nickname::new("", "schnappi", "schnappi"),
        Nickname::new("", "schnubber", "schnubber"),
        Nickname::new("", "", "schnubbi"),
        Nickname::new("", "schnüff", ""),
        Nickname::new("", "schnüffel", ""),
        Nickname::new("", "schnuffel", "schnuffel"),
        Nickname::new("", "", "schnufferling"),
        Nickname::new("", "", "schnuffi"),
        Nickname::new("", "", "schnuggi"),
        Nickname::new("", "", "schnüpsel"),
        Nickname::new("schnüpstastisches", "schnüps", "schnüps"),
        Nickname::new("", "schnürps", "schnürps"),
        Nickname::new("schnusiges", "schnusig", ""),
        Nickname::new("", "", "stück"),
        Nickname::new("süsses", "", ""),
        Nickname::new("", "tier", "tier"),
        Nickname::new("", "wuffel", "wuffel"),
        Nickname::new("", "wuff", "wuff"),
        Nickname::new("", "wuschel", "wuscheli"),
        Nickname::new("wutziges", "wutz", "wutz"),
    ];

    let adjective = &mut nicknames[rng.gen_range(0..nicknames.len() - 1)]
        .adjective
        .to_owned();
    let prefix = &nicknames[rng.gen_range(0..nicknames.len() - 1)].prefix;

    let mut noun = "";
    while noun == "" {
        noun = &nicknames[rng.gen_range(0..nicknames.len() - 1)].noun;
    }

    if adjective != "" {
        adjective.push_str(" ")
    }

    // if prefix == "" {
    //     noun = &to_title_case(noun);
    // } else {
    //     prefix = &to_title_case(prefix);
    // }

    println!();
    println!(
        "    ___
 __/_  `.  .-\"\"\"-.
 \\_,` | \\-'  /   )`-')
  \"\") `\"`    \\  ((`\"`
 ___Y  ,    .'7 /|
(_,___/...-` (_/_/"
    );
    println!();
    println!("{}{}{}", adjective, prefix, noun)
}

// fn to_title_case(s: &str) -> String {
//     format!("{}{}", (&s[..1].to_string()).to_uppercase(), &s[1..])
// }
