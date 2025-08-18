use rand::prelude::IndexedRandom;
use rand::rng;

struct Nickname {
    adjective: &'static str,
    prefix: &'static str,
    noun: &'static str,
}

fn to_title_case(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for (i, word) in s.split_whitespace().enumerate() {
        if i > 0 { result.push(' '); }
        let mut chars = word.chars();
        if let Some(first) = chars.next() {
            result.extend(first.to_uppercase());
            result.push_str(&chars.as_str().to_lowercase());
        }
    }
    result
}

const NICKNAMES: &[Nickname] = &[
    Nickname { adjective: "agent",           prefix: "",           noun: ""            },
    Nickname { adjective: "bädäss",          prefix: "bädäss",     noun: "bädäss"      },
    Nickname { adjective: "",                prefix: "bären",      noun: "bär"         },
    Nickname { adjective: "",                prefix: "butz",       noun: "butz"        },
    Nickname { adjective: "dickes",          prefix: "",           noun: ""            },
    Nickname { adjective: "dummbatziges",    prefix: "dummbatz",   noun: "dummbatz"    },
    Nickname { adjective: "freches",         prefix: "",           noun: ""            },
    Nickname { adjective: "general",         prefix: "",           noun: ""            },
    Nickname { adjective: "grosses",         prefix: "gross",      noun: ""            },
    Nickname { adjective: "",                prefix: "gschöpf",    noun: "gschöpf"     },
    Nickname { adjective: "gutes",           prefix: "",           noun: ""            },
    Nickname { adjective: "härziges",        prefix: "",           noun: ""            },
    Nickname { adjective: "huere",           prefix: "",           noun: ""            },
    Nickname { adjective: "",                prefix: "hunde",      noun: "hund"        },
    Nickname { adjective: "",                prefix: "",           noun: "hündelus"    },
    Nickname { adjective: "",                prefix: "",           noun: "hundeli"     },
    Nickname { adjective: "",                prefix: "",           noun: "iisbär"      },
    Nickname { adjective: "kleines",         prefix: "",           noun: ""            },
    Nickname { adjective: "kluges",          prefix: "",           noun: ""            },
    Nickname { adjective: "",                prefix: "kuschel",    noun: "kuscheli"    },
    Nickname { adjective: "",                prefix: "lieblings",  noun: ""            },
    Nickname { adjective: "madame",          prefix: "",           noun: "madame"      },
    Nickname { adjective: "",                prefix: "mause",      noun: "maus"        },
    Nickname { adjective: "",                prefix: "mäuse",      noun: "mäuschen"    },
    Nickname { adjective: "",                prefix: "mausi",      noun: "mausi"       },
    Nickname { adjective: "",                prefix: "",           noun: "meite"       },
    Nickname { adjective: "",                prefix: "",           noun: "meiteli"     },
    Nickname { adjective: "",                prefix: "meiti",      noun: "meiti"       },
    Nickname { adjective: "missus",          prefix: "",           noun: ""            },
    Nickname { adjective: "",                prefix: "monster",    noun: "monster"     },
    Nickname { adjective: "",                prefix: "mutz",       noun: "mutz"        },
    Nickname { adjective: "",                prefix: "näsi",       noun: "näsi"        },
    Nickname { adjective: "",                prefix: "nöff",       noun: "nöff"        },
    Nickname { adjective: "",                prefix: "puffel",     noun: "puffel"      },
    Nickname { adjective: "",                prefix: "puff",       noun: "puff"        },
    Nickname { adjective: "",                prefix: "",           noun: "pupsi"       },
    Nickname { adjective: "",                prefix: "quietsch",   noun: ""            },
    Nickname { adjective: "richtiges",       prefix: "",           noun: ""            },
    Nickname { adjective: "riktastisches",   prefix: "rika",       noun: "rika"        },
    Nickname { adjective: "",                prefix: "",           noun: "schatzi"     },
    Nickname { adjective: "",                prefix: "schatz",     noun: "schatz"      },
    Nickname { adjective: "",                prefix: "schmatz",    noun: "schmatz"     },
    Nickname { adjective: "",                prefix: "schnabautz", noun: "schnabautz"  },
    Nickname { adjective: "",                prefix: "schnabber",  noun: "schnabber"   },
    Nickname { adjective: "",                prefix: "schnapp",    noun: ""            },
    Nickname { adjective: "",                prefix: "schnappi",   noun: "schnappi"    },
    Nickname { adjective: "",                prefix: "schnapps",   noun: "schnapps"    },
    Nickname { adjective: "",                prefix: "schnubber",  noun: "schnubber"   },
    Nickname { adjective: "",                prefix: "schnübbär",  noun: "schnübbär"   },
    Nickname { adjective: "",                prefix: "",           noun: "schnubbi"    },
    Nickname { adjective: "",                prefix: "schnüff",    noun: ""            },
    Nickname { adjective: "",                prefix: "schnüffel",  noun: ""            },
    Nickname { adjective: "",                prefix: "schnuffel",  noun: "schnuffel"   },
    Nickname { adjective: "",                prefix: "",           noun: "schnufferling"},
    Nickname { adjective: "",                prefix: "",           noun: "schnuffi"    },
    Nickname { adjective: "",                prefix: "schnuff",    noun: "schnuff"     },
    Nickname { adjective: "",                prefix: "",           noun: "schnuggi"    },
    Nickname { adjective: "",                prefix: "",           noun: "schnüpsel"   },
    Nickname { adjective: "schnüpstastisches", prefix: "schnüps",  noun: "schnüps"     },
    Nickname { adjective: "",                prefix: "schnürps",   noun: "schnürps"    },
    Nickname { adjective: "schnusiges",      prefix: "schnusig",   noun: ""            },
    Nickname { adjective: "",                prefix: "",           noun: "stück"       },
    Nickname { adjective: "süsses",          prefix: "",           noun: ""            },
    Nickname { adjective: "",                prefix: "tier",       noun: "tier"        },
    Nickname { adjective: "",                prefix: "wuffel",     noun: "wuffel"      },
    Nickname { adjective: "",                prefix: "wuff",       noun: "wuff"        },
    Nickname { adjective: "",                prefix: "wuschel",    noun: "wuscheli"    },
    Nickname { adjective: "wutziges",        prefix: "wutz",       noun: "wutz"        },
];

fn main() {
    let mut rng = rng();

    let adjective = NICKNAMES.choose(&mut rng).unwrap().adjective;
    let prefix = NICKNAMES.choose(&mut rng).unwrap().prefix;

    let nouns: Vec<&str> = NICKNAMES.iter().map(|n| n.noun).filter(|n| !n.is_empty()).collect();
    let noun = nouns.choose(&mut rng).unwrap();

    let sep = if adjective.is_empty() { "" } else { " " };

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
    let final_nickname = format!("{adjective}{sep}{prefix}{noun}");
    println!("{}", to_title_case(&final_nickname));
}
