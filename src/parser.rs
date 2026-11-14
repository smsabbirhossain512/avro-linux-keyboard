pub fn parse_avro(text: &str) -> String {
    let compounds: &[(&str, &str)] = &[
        ("ngkh", "ঙ্খ"), ("nggh", "ঙ্ঘ"), ("nchh", "ঞ্ছ"),
        ("shch", "শ্ছ"), ("ShTh", "ষ্ঠ"),
        ("ksh", "ক্ষ"), ("kkh", "ক্ষ"), ("jng", "জ্ঞ"),
        ("ngk", "ঙ্ক"), ("ngg", "ঙ্গ"), ("nch", "ঞ্চ"), ("njh", "ঞ্ঝ"),
        ("tth", "ত্থ"), ("ddh", "দ্ধ"), ("nth", "ন্থ"), ("ndh", "ন্ধ"),
        ("mph", "ম্ফ"), ("mbh", "ম্ভ"), ("shc", "শ্চ"), ("shn", "শ্ন"),
        ("shw", "শ্ব"), ("shm", "শ্ম"), ("shr", "শ্র"), ("shl", "শ্ল"),
        ("Skr", "ষ্ক্র"), ("ShT", "ষ্ট"), ("ShN", "ষ্ণ"), ("Sph", "ষ্ফ"),
        ("skr", "স্ক্র"), ("skh", "স্খ"), ("str", "স্ত্র"), ("sth", "স্থ"),
        ("sph", "স্ফ"), ("bhr", "ভ্র"), ("dhr", "ধ্র"), ("ktr", "ক্ত্র"),
        ("bdh", "ব্ধ"), ("rri", "ঋ"), ("t``", "ৎ"),
        ("gg", "জ্ঞ"), ("gG", "জ্ঞ"), ("nc", "ঞ্চ"), ("nj", "ঞ্জ"),
        ("tt", "ত্ত"), ("tn", "ত্ন"), ("tm", "ত্ম"), ("tw", "ত্ব"),
        ("dd", "দ্দ"), ("dw", "দ্ব"), ("dm", "দ্ম"),
        ("nt", "ন্ত"), ("nd", "ন্দ"), ("nn", "ন্ন"), ("nw", "ন্ব"), ("nm", "ন্ম"),
        ("mp", "ম্প"), ("mb", "ম্ব"), ("mm", "ম্ম"), ("ml", "ম্ল"),
        ("lk", "ল্ক"), ("lg", "ল্গ"), ("lt", "ল্ট"), ("ld", "ল্ড"),
        ("lp", "ল্প"), ("lm", "ল্ম"), ("ll", "ল্ল"),
        ("Sk", "ষ্ক"), ("Sp", "ষ্প"), ("Sm", "ষ্ম"),
        ("sk", "স্ক"), ("st", "স্ত"), ("sn", "স্ন"), ("sp", "স্প"),
        ("sw", "স্ব"), ("sm", "স্ম"), ("sl", "স্ল"),
        ("hm", "হ্ম"), ("hn", "হ্ণ"), ("hl", "হ্ল"), ("hr", "হ্র"), ("hw", "হ্ব"),
        ("tr", "ত্র"), ("dr", "দ্র"), ("pr", "প্র"), ("br", "ব্র"), ("mr", "ম্র"),
        ("bd", "ব্দ"), ("bj", "ব্জ"), ("bb", "ব্ব"), ("bl", "ব্ল"),
        ("kt", "ক্ত"), ("kr", "ক্র"), ("kl", "ক্ল"), ("kw", "ক্ব"), ("kk", "ক্ক"),
        ("gd", "গ্ধ"), ("gn", "গ্ন"), ("gr", "গ্র"), ("gl", "গ্ল"), ("gw", "গ্ব"),
        ("x", "ক্স"),
    ];

    let consonants: &[(&str, &str)] = &[
        ("kh", "খ"), ("gh", "ঘ"), ("Ng", "ঙ"), ("ng", "ং"),
        ("ch", "ছ"), ("jh", "ঝ"), ("NG", "ঞ"),
        ("Th", "ঠ"), ("Dh", "ঢ"), ("th", "থ"), ("dh", "ধ"),
        ("ph", "ফ"), ("bh", "ভ"), ("sh", "শ"), ("Rh", "ঢ়"),
        ("k", "ক"), ("q", "ক"), ("Q", "ক"), ("g", "গ"),
        ("c", "চ"), ("j", "জ"),
        ("T", "ট"), ("D", "ড"), ("N", "ণ"),
        ("t", "ত"), ("d", "দ"), ("n", "ন"),
        ("p", "প"), ("f", "ফ"), ("b", "ব"), ("v", "ভ"), ("m", "ম"),
        ("z", "য"), ("y", "য়"), ("r", "র"), ("l", "ল"),
        ("S", "ষ"), ("s", "স"), ("h", "হ"),
        ("R", "ড়"), ("Y", "য়"), ("w", "ও"), ("W", "ও"),
        ("H", "ঃ"), ("^", "ঁ"),
    ];

    let vowels: &[(&str, &str, &str)] = &[
        ("ee", "ঈ", "ী"), ("oo", "ঊ", "ূ"),
        ("OI", "ঐ", "ৈ"), ("OU", "ঔ", "ৌ"),
        ("o", "অ", ""), ("a", "আ", "া"), ("i", "ই", "ি"), ("I", "ঈ", "ী"),
        ("u", "উ", "ু"), ("U", "ঊ", "ূ"), ("e", "এ", "ে"), ("O", "ও", "ো"),
    ];

    let mut res: Vec<String> = Vec::new();
    let mut i = 0;
    let bytes = text.as_bytes();
    let n = bytes.len();
    let mut prev_type = "none";

    while i < n {
        let rem = &text[i..];
        if rem.starts_with("..") {
            res.push(".".to_string());
            prev_type = "other";
            i += 2;
            continue;
        }
        if rem.starts_with('.') {
            res.push("।".to_string());
            prev_type = "other";
            i += 1;
            continue;
        }
        if rem.starts_with(",,") {
            res.push("্".to_string());
            prev_type = "hasanta";
            i += 2;
            continue;
        }
        if let Some(ch) = rem.chars().next() {
            if ch.is_ascii_digit() {
                let bn_digit = match ch {
                    '0' => "০", '1' => "১", '2' => "২", '3' => "৩", '4' => "৪",
                    '5' => "৫", '6' => "৬", '7' => "৭", '8' => "৮", _ => "৯",
                };
                res.push(bn_digit.to_string());
                prev_type = "num";
                i += 1;
                continue;
            }
        }

        if rem.starts_with("rri") {
            if prev_type == "cons" {
                res.push("ৃ".to_string());
            } else {
                res.push("ঋ".to_string());
            }
            prev_type = "vowel";
            i += 3;
            continue;
        }

        if prev_type != "cons" && (rem.starts_with("wa") || rem.starts_with("Wa")) {
            res.push("ওয়া".to_string());
            prev_type = "vowel";
            i += 2;
            continue;
        }

        let mut matched = false;

        for &(k, v) in compounds {
            if rem.starts_with(k) {
                if prev_type == "cons" {
                    res.push(format!("্{}", v));
                } else {
                    res.push(v.to_string());
                }
                prev_type = "cons";
                i += k.len();
                matched = true;
                break;
            }
        }
        if matched { continue; }

        for &(k, v) in consonants {
            if rem.starts_with(k) {
                if prev_type == "cons" {
                    if k == "y" {
                        res.push("্য".to_string());
                    } else if k == "w" || k == "W" {
                        res.push("্ব".to_string());
                    } else if k == "r" && res.last().map(|s| s.as_str()) != Some("র") {
                        res.push("্র".to_string());
                    } else if res.last().map(|s| s.as_str()) == Some("র") && k != "y" {
                        if let Some(last) = res.last_mut() {
                            *last = format!("র্{}", v);
                        }
                    } else {
                        res.push(format!("্{}", v));
                    }
                } else {
                    res.push(v.to_string());
                }
                prev_type = if matches!(k, "ng" | "H" | "^") || ((k == "w" || k == "W") && prev_type != "cons") {
                    "vowel"
                } else {
                    "cons"
                };
                i += k.len();
                matched = true;
                break;
            }
        }
        if matched { continue; }

        for &(k, full_v, kar_v) in vowels {
            if rem.starts_with(k) {
                if prev_type == "cons" {
                    res.push(kar_v.to_string());
                } else {
                    res.push(full_v.to_string());
                }
                prev_type = "vowel";
                i += k.len();
                matched = true;
                break;
            }
        }
        if matched { continue; }

        if let Some(ch) = rem.chars().next() {
            res.push(ch.to_string());
            prev_type = "other";
            i += ch.len_utf8();
        }
    }

    res.concat()
}
